//! [`CompressionSession`] — hardware H.264/HEVC/ProRes encoder.

use core::ffi::c_void;
#[cfg(feature = "async")]
use core::future::Future;
use core::ptr;
#[cfg(feature = "async")]
use core::task::Poll;
use core::task::Waker;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError};

use apple_cf::cf::{CFDictionary, CFNumber, CFString, CFType};
use apple_cf::cm::{CMSampleBuffer, CMTime};
use apple_cf::cv::{CVPixelBuffer, CVPixelBufferPool};
use apple_cf::iosurface::IOSurface;
use doom_fish_utils::panic_safe::catch_user_panic;

use crate::error::VTError;
use crate::ffi;
use crate::multipass::MultiPassStorage;
use crate::session::{self, Codec};
use crate::tagged_buffer_group::TaggedBufferGroup;

/// One encoded frame produced by [`CompressionSession::encode`].
///
/// Wraps the encoder's `CMSampleBuffer` as a safe [`apple_cf::cm::CMSampleBuffer`]
/// so downstream crates (e.g. `avassetwriter-rs`) can hand it off zero-copy
/// without dealing with raw `*mut c_void` pointers.
pub struct EncodedFrame {
    /// Encoded bitstream bytes. For H.264/HEVC these are AVCC-style NAL units,
    /// each preceded by a big-endian length field (no Annex B start codes); the
    /// field size and the SPS/PPS/VPS parameter sets come from the format
    /// description (`CMFormatDescription::video_parameter_sets`). For `ProRes`,
    /// the frame data.
    pub data: Vec<u8>,
    /// Presentation timestamp of the frame.
    pub presentation_time: CMTime,
    /// Encoder hint flags (e.g. dropped, asynchronous).
    pub info_flags: u32,
    /// Underlying `CoreMedia` sample buffer. `None` for dropped frames.
    sample_buffer: Option<CMSampleBuffer>,
}

impl EncodedFrame {
    /// Borrow the encoded sample buffer for cross-crate hand-off (e.g. to
    /// `avassetwriter::Writer::append_sample`). Returns `None` for dropped
    /// frames.
    #[must_use]
    pub const fn cm_sample_buffer(&self) -> Option<&apple_cf::cm::CMSampleBuffer> {
        self.sample_buffer.as_ref()
    }

    /// Raw `CMSampleBufferRef` for code that bypasses `apple_cf` and talks
    /// directly to a Swift bridge via `extern "C"`.
    ///
    /// Returns `null` for dropped frames. Do **not** call `CFRelease` on
    /// the returned pointer — ownership stays with this `EncodedFrame`.
    #[must_use]
    pub fn cm_sample_buffer_ptr(&self) -> ffi::CMSampleBufferRef {
        self.sample_buffer
            .as_ref()
            .map_or(core::ptr::null_mut::<c_void>().cast(), |sample_buffer| {
                sample_buffer.as_ptr().cast()
            })
    }

    fn from_output(output: FrameOutput, source_time: CMTime) -> Result<Self, VTError> {
        let Some(sample_buffer) = output.sample_buffer else {
            return Ok(Self {
                data: Vec::new(),
                presentation_time: source_time,
                info_flags: output.info_flags,
                sample_buffer: None,
            });
        };
        let block_buffer =
            unsafe { ffi::CMSampleBufferGetDataBuffer(sample_buffer.as_ptr().cast()) };
        if block_buffer.is_null() {
            return Err(VTError::EncoderCallback(-2));
        }
        let len = unsafe { ffi::CMBlockBufferGetDataLength(block_buffer) };
        let mut data = vec![0u8; len];
        let copy_status = unsafe {
            ffi::CMBlockBufferCopyDataBytes(
                block_buffer,
                0,
                len,
                data.as_mut_ptr().cast::<c_void>(),
            )
        };
        if copy_status != 0 {
            return Err(VTError::EncoderCallback(copy_status));
        }
        Ok(Self {
            data,
            presentation_time: sample_buffer.presentation_timestamp(),
            info_flags: output.info_flags,
            sample_buffer: Some(sample_buffer),
        })
    }
}

impl Clone for EncodedFrame {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            presentation_time: self.presentation_time,
            info_flags: self.info_flags,
            sample_buffer: self.sample_buffer.clone(),
        }
    }
}

impl core::fmt::Debug for EncodedFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EncodedFrame")
            .field("len", &self.data.len())
            .field("presentation_time", &self.presentation_time)
            .field("info_flags", &self.info_flags)
            .field("sample_buffer", &self.sample_buffer)
            .finish()
    }
}

/// Builder for [`CompressionSession`].
///
/// # Example
///
/// ```rust,no_run
/// use videotoolbox::compression::CompressionSession;
/// use videotoolbox::session::Codec;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let session = CompressionSession::builder(1920, 1080, Codec::H264)
///     .with_real_time(true)
///     .with_average_bit_rate(8_000_000)
///     .with_expected_frame_rate(60.0)
///     .with_max_keyframe_interval(120)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct CompressionSessionBuilder {
    width: i32,
    height: i32,
    codec: Codec,
    real_time: Option<bool>,
    allow_frame_reordering: Option<bool>,
    average_bit_rate: Option<i32>,
    expected_frame_rate: Option<f64>,
    max_keyframe_interval: Option<i32>,
    quality: Option<f32>,
    profile_level: Option<ProfileLevel>,
    hardware_acceleration: Option<HardwareAcceleration>,
    encoder_id: Option<String>,
    low_latency_rate_control: Option<bool>,
    encoder_gpu: Option<EncoderGpu>,
    source_pixel_buffer_attributes: Option<CFDictionary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HardwareAcceleration {
    Preferred,
    Required,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncoderGpu {
    Preferred(u64),
    Required(u64),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FrameProperties {
    force_key_frame: bool,
}

impl FrameProperties {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            force_key_frame: false,
        }
    }

    #[must_use]
    pub const fn with_force_key_frame(mut self, force_key_frame: bool) -> Self {
        self.force_key_frame = force_key_frame;
        self
    }

    #[must_use]
    pub fn to_dictionary(self) -> Option<CFDictionary> {
        let mut pairs = Vec::new();
        if self.force_key_frame {
            pairs.push((
                unsafe { ffi::kVTEncodeFrameOptionKey_ForceKeyFrame },
                cf_boolean(true),
            ));
        }
        cf_dictionary(&pairs)
    }
}

fn cf_boolean(value: bool) -> ffi::CFTypeRef {
    let value = unsafe {
        if value {
            ffi::kCFBooleanTrue
        } else {
            ffi::kCFBooleanFalse
        }
    };
    value.cast()
}

fn cf_dictionary(pairs: &[(ffi::CFStringRef, ffi::CFTypeRef)]) -> Option<CFDictionary> {
    if pairs.is_empty() {
        return None;
    }
    let dictionary = unsafe {
        ffi::CFDictionaryCreateMutable(
            ffi::kCFAllocatorDefault,
            0,
            (&raw const ffi::kCFTypeDictionaryKeyCallBacks).cast(),
            (&raw const ffi::kCFTypeDictionaryValueCallBacks).cast(),
        )
    };
    if dictionary.is_null() {
        return None;
    }
    for (key, value) in pairs {
        unsafe { ffi::CFDictionarySetValue(dictionary, key.cast(), value.cast()) };
    }
    unsafe { CFDictionary::from_raw(dictionary.cast()) }
}

macro_rules! define_profile_levels {
    ($($variant:ident => $ffi_const:ident),+ $(,)?) => {
        /// Encoded profile/level for the underlying codec. Maps to
        /// `kVTProfileLevel_*` `CFStringRef` constants.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum ProfileLevel {
            $(
                $variant,
            )+
        }

        impl ProfileLevel {
            /// Every profile level this build knows about, in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            /// The variant's Rust name, e.g. `"H264HighAutoLevel"`.
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => stringify!($variant),
                    )+
                }
            }

            /// Look a profile up by [`Self::name`], ignoring case and any
            /// `_`/`-`/`.` separators, so `"h264 high auto level"` and
            /// `"H264High_AutoLevel"` both resolve.
            #[must_use]
            pub fn from_name(name: &str) -> Option<Self> {
                fn normalize(s: &str) -> String {
                    s.chars()
                        .filter(|c| c.is_ascii_alphanumeric())
                        .map(|c| c.to_ascii_lowercase())
                        .collect()
                }
                let wanted = normalize(name);
                Self::ALL.iter().copied().find(|p| normalize(p.name()) == wanted)
            }

            pub(crate) fn as_cf_string(self) -> ffi::CFStringRef {
                // SAFETY: FFI constants are statically defined by Apple's VideoToolbox SDK.
                // Returning them as immutable references is safe.
                unsafe {
                    match self {
                        $(
                            Self::$variant => ffi::$ffi_const,
                        )+
                    }
                }
            }
        }
    };
}

define_profile_levels! {
    H263Profile0Level10 => kVTProfileLevel_H263_Profile0_Level10,
    H263Profile0Level45 => kVTProfileLevel_H263_Profile0_Level45,
    H263Profile3Level45 => kVTProfileLevel_H263_Profile3_Level45,
    H264Baseline1_3 => kVTProfileLevel_H264_Baseline_1_3,
    H264Baseline3_0 => kVTProfileLevel_H264_Baseline_3_0,
    H264Baseline3_1 => kVTProfileLevel_H264_Baseline_3_1,
    H264Baseline3_2 => kVTProfileLevel_H264_Baseline_3_2,
    H264Baseline4_0 => kVTProfileLevel_H264_Baseline_4_0,
    H264Baseline4_1 => kVTProfileLevel_H264_Baseline_4_1,
    H264Baseline4_2 => kVTProfileLevel_H264_Baseline_4_2,
    H264Baseline5_0 => kVTProfileLevel_H264_Baseline_5_0,
    H264Baseline5_1 => kVTProfileLevel_H264_Baseline_5_1,
    H264Baseline5_2 => kVTProfileLevel_H264_Baseline_5_2,
    H264BaselineAutoLevel => kVTProfileLevel_H264_Baseline_AutoLevel,
    H264ConstrainedBaselineAutoLevel => kVTProfileLevel_H264_ConstrainedBaseline_AutoLevel,
    H264ConstrainedHighAutoLevel => kVTProfileLevel_H264_ConstrainedHigh_AutoLevel,
    H264Extended5_0 => kVTProfileLevel_H264_Extended_5_0,
    H264ExtendedAutoLevel => kVTProfileLevel_H264_Extended_AutoLevel,
    H264High3_0 => kVTProfileLevel_H264_High_3_0,
    H264High3_1 => kVTProfileLevel_H264_High_3_1,
    H264High3_2 => kVTProfileLevel_H264_High_3_2,
    H264High4_0 => kVTProfileLevel_H264_High_4_0,
    H264High4_1 => kVTProfileLevel_H264_High_4_1,
    H264High4_2 => kVTProfileLevel_H264_High_4_2,
    H264High5_0 => kVTProfileLevel_H264_High_5_0,
    H264High5_1 => kVTProfileLevel_H264_High_5_1,
    H264High5_2 => kVTProfileLevel_H264_High_5_2,
    H264HighAutoLevel => kVTProfileLevel_H264_High_AutoLevel,
    H264Main3_0 => kVTProfileLevel_H264_Main_3_0,
    H264Main3_1 => kVTProfileLevel_H264_Main_3_1,
    H264Main3_2 => kVTProfileLevel_H264_Main_3_2,
    H264Main4_0 => kVTProfileLevel_H264_Main_4_0,
    H264Main4_1 => kVTProfileLevel_H264_Main_4_1,
    H264Main4_2 => kVTProfileLevel_H264_Main_4_2,
    H264Main5_0 => kVTProfileLevel_H264_Main_5_0,
    H264Main5_1 => kVTProfileLevel_H264_Main_5_1,
    H264Main5_2 => kVTProfileLevel_H264_Main_5_2,
    H264MainAutoLevel => kVTProfileLevel_H264_Main_AutoLevel,
    HEVCMainAutoLevel => kVTProfileLevel_HEVC_Main_AutoLevel,
    HEVCMain10AutoLevel => kVTProfileLevel_HEVC_Main10_AutoLevel,
    HEVCMain42210AutoLevel => kVTProfileLevel_HEVC_Main42210_AutoLevel,
    HEVCMonochromeAutoLevel => kVTProfileLevel_HEVC_Monochrome_AutoLevel,
    HEVCMonochrome10AutoLevel => kVTProfileLevel_HEVC_Monochrome10_AutoLevel,
    MP4VAdvancedSimpleL0 => kVTProfileLevel_MP4V_AdvancedSimple_L0,
    MP4VAdvancedSimpleL1 => kVTProfileLevel_MP4V_AdvancedSimple_L1,
    MP4VAdvancedSimpleL2 => kVTProfileLevel_MP4V_AdvancedSimple_L2,
    MP4VAdvancedSimpleL3 => kVTProfileLevel_MP4V_AdvancedSimple_L3,
    MP4VAdvancedSimpleL4 => kVTProfileLevel_MP4V_AdvancedSimple_L4,
    MP4VMainL2 => kVTProfileLevel_MP4V_Main_L2,
    MP4VMainL3 => kVTProfileLevel_MP4V_Main_L3,
    MP4VMainL4 => kVTProfileLevel_MP4V_Main_L4,
    MP4VSimpleL0 => kVTProfileLevel_MP4V_Simple_L0,
    MP4VSimpleL1 => kVTProfileLevel_MP4V_Simple_L1,
    MP4VSimpleL2 => kVTProfileLevel_MP4V_Simple_L2,
    MP4VSimpleL3 => kVTProfileLevel_MP4V_Simple_L3,
}

impl CompressionSessionBuilder {
    #[must_use]
    pub const fn new(width: i32, height: i32, codec: Codec) -> Self {
        Self {
            width,
            height,
            codec,
            real_time: None,
            allow_frame_reordering: None,
            average_bit_rate: None,
            expected_frame_rate: None,
            max_keyframe_interval: None,
            quality: None,
            profile_level: None,
            hardware_acceleration: None,
            encoder_id: None,
            low_latency_rate_control: None,
            encoder_gpu: None,
            source_pixel_buffer_attributes: None,
        }
    }

    /// Hint to the encoder that frames are arriving in real-time (e.g. live
    /// capture) and it should prioritise low latency over compression efficiency.
    #[must_use]
    pub const fn with_real_time(mut self, real_time: bool) -> Self {
        self.real_time = Some(real_time);
        self
    }

    /// Allow B-frame reordering (better compression, higher latency).
    #[must_use]
    pub const fn with_allow_frame_reordering(mut self, allow: bool) -> Self {
        self.allow_frame_reordering = Some(allow);
        self
    }

    /// Target average bitrate in bits-per-second.
    #[must_use]
    pub const fn with_average_bit_rate(mut self, bps: i32) -> Self {
        self.average_bit_rate = Some(bps);
        self
    }

    /// Expected source frame rate. Helps the encoder size its rate-control window.
    #[must_use]
    pub const fn with_expected_frame_rate(mut self, fps: f64) -> Self {
        self.expected_frame_rate = Some(fps);
        self
    }

    /// Force a keyframe at most every `n` frames.
    #[must_use]
    pub const fn with_max_keyframe_interval(mut self, n: i32) -> Self {
        self.max_keyframe_interval = Some(n);
        self
    }

    /// Encoding quality hint, `0.0..=1.0`. `0.0` minimum quality / size,
    /// `1.0` maximum. Wraps `kVTCompressionPropertyKey_Quality`.
    #[must_use]
    pub const fn with_quality(mut self, quality: f32) -> Self {
        self.quality = Some(quality);
        self
    }

    /// Profile/level for the encoded stream. See [`ProfileLevel`].
    /// Wraps `kVTCompressionPropertyKey_ProfileLevel`.
    #[must_use]
    pub const fn with_profile_level(mut self, profile: ProfileLevel) -> Self {
        self.profile_level = Some(profile);
        self
    }

    #[must_use]
    pub const fn with_hardware_acceleration(mut self, acceleration: HardwareAcceleration) -> Self {
        self.hardware_acceleration = Some(acceleration);
        self
    }

    #[must_use]
    pub fn with_encoder_id(mut self, encoder_id: impl Into<String>) -> Self {
        self.encoder_id = Some(encoder_id.into());
        self
    }

    #[must_use]
    pub const fn with_low_latency_rate_control(mut self, enabled: bool) -> Self {
        self.low_latency_rate_control = Some(enabled);
        self
    }

    #[must_use]
    pub const fn with_encoder_gpu(mut self, gpu: EncoderGpu) -> Self {
        self.encoder_gpu = Some(gpu);
        self
    }

    #[must_use]
    pub fn with_source_pixel_buffer_attributes(mut self, attributes: CFDictionary) -> Self {
        self.source_pixel_buffer_attributes = Some(attributes);
        self
    }

    /// Construct the session and apply all configured properties.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::InvalidArgument`] if width/height are non-positive,
    /// [`VTError::SessionCreateFailed`] if `VideoToolbox` refuses to instantiate,
    /// or [`VTError::SetPropertyFailed`] if any property is rejected.
    pub fn build(self) -> Result<CompressionSession, VTError> {
        if self.width <= 0 || self.height <= 0 {
            return Err(VTError::InvalidArgument(format!(
                "width/height must be positive (got {}x{})",
                self.width, self.height
            )));
        }
        CompressionSession::new_internal(&self)
    }

    fn encoder_specification(&self) -> Option<CFDictionary> {
        let encoder_id = self.encoder_id.as_deref().map(CFString::new);
        let encoder_gpu = self.encoder_gpu.map(|gpu| match gpu {
            EncoderGpu::Preferred(registry_id) => (
                unsafe { ffi::kVTVideoEncoderSpecification_PreferredEncoderGPURegistryID },
                CFNumber::from_u64(registry_id),
            ),
            EncoderGpu::Required(registry_id) => (
                unsafe { ffi::kVTVideoEncoderSpecification_RequiredEncoderGPURegistryID },
                CFNumber::from_u64(registry_id),
            ),
        });

        let mut pairs = Vec::new();
        match self.hardware_acceleration {
            Some(HardwareAcceleration::Preferred) => pairs.push((
                unsafe { ffi::kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder },
                cf_boolean(true),
            )),
            Some(HardwareAcceleration::Required) => pairs.push((
                unsafe { ffi::kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder },
                cf_boolean(true),
            )),
            Some(HardwareAcceleration::Disabled) => pairs.push((
                unsafe { ffi::kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder },
                cf_boolean(false),
            )),
            None => {}
        }
        if let Some(encoder_id) = &encoder_id {
            pairs.push((
                unsafe { ffi::kVTVideoEncoderSpecification_EncoderID },
                encoder_id.as_ptr().cast_const(),
            ));
        }
        if let Some(enabled) = self.low_latency_rate_control {
            pairs.push((
                unsafe { ffi::kVTVideoEncoderSpecification_EnableLowLatencyRateControl },
                cf_boolean(enabled),
            ));
        }
        if let Some((key, registry_id)) = &encoder_gpu {
            pairs.push((*key, registry_id.as_ptr().cast_const()));
        }
        cf_dictionary(&pairs)
    }
}

/// Hardware-accelerated video compression session.
///
/// Construct via [`CompressionSession::builder`]. Each session owns a native
/// `VTCompressionSessionRef`; the underlying encoder is invalidated on drop.
pub struct CompressionSession {
    session: ffi::VTCompressionSessionRef,
}

// SAFETY: the session handle may move to another thread; the output callback
// only touches the per-frame slot it is handed, which is `Send + Sync`.
unsafe impl Send for CompressionSession {}

struct FrameOutput {
    info_flags: ffi::VTEncodeInfoFlags,
    sample_buffer: Option<CMSampleBuffer>,
}

#[derive(Default)]
struct FrameState {
    output: Option<Result<FrameOutput, VTError>>,
    waker: Option<Waker>,
}

struct FrameSlot {
    native: AtomicBool,
    state: Mutex<FrameState>,
    completed: Condvar,
}

impl FrameSlot {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            native: AtomicBool::new(true),
            state: Mutex::new(FrameState::default()),
            completed: Condvar::new(),
        })
    }

    fn native_ref_con(self: &Arc<Self>) -> *mut c_void {
        Arc::into_raw(Arc::clone(self)).cast_mut().cast()
    }

    fn submit(submit: impl FnOnce(*mut c_void) -> ffi::OSStatus) -> Result<Arc<Self>, VTError> {
        let slot = Self::new();
        let ref_con = slot.native_ref_con();
        let status = submit(ref_con);
        if status != 0 {
            drop(unsafe { Self::release_native(ref_con) });
            return Err(VTError::EncodeFailed(status));
        }
        Ok(slot)
    }

    unsafe fn release_native(ref_con: *mut c_void) -> Option<Arc<Self>> {
        let slot = unsafe { ref_con.cast::<Self>().as_ref() }?;
        if slot.native.swap(false, Ordering::AcqRel) {
            Some(unsafe { Arc::from_raw(ref_con.cast::<Self>().cast_const()) })
        } else {
            None
        }
    }

    fn complete(&self, result: Result<FrameOutput, VTError>) {
        let waker = {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            if state.output.is_none() {
                state.output = Some(result);
            }
            state.waker.take()
        };
        self.completed.notify_all();
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    fn take(&self) -> Option<Result<FrameOutput, VTError>> {
        self.state
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .output
            .take()
    }

    fn wait(&self) -> Result<FrameOutput, VTError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        loop {
            if let Some(output) = state.output.take() {
                return output;
            }
            state = self
                .completed
                .wait(state)
                .unwrap_or_else(PoisonError::into_inner);
        }
    }

    #[cfg(feature = "async")]
    fn poll_output(&self, waker: &Waker) -> Option<Result<FrameOutput, VTError>> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let output = state.output.take();
        if output.is_none() {
            match &state.waker {
                Some(existing) if existing.will_wake(waker) => {}
                _ => state.waker = Some(waker.clone()),
            }
        }
        output
    }
}

impl CompressionSession {
    /// Convenience: start a builder.
    #[must_use]
    pub const fn builder(width: i32, height: i32, codec: Codec) -> CompressionSessionBuilder {
        CompressionSessionBuilder::new(width, height, codec)
    }

    /// CoreFoundation type identifier for `VTCompressionSession`.
    #[must_use]
    pub fn type_id() -> usize {
        // SAFETY: `VTCompressionSessionGetTypeID` is a standard Apple SDK function
        // that returns a static type ID. Safe to call from any thread.
        unsafe { ffi::VTCompressionSessionGetTypeID() }
    }

    /// Returns `true` when the current system advertises stereo MV-HEVC
    /// encode support.
    #[must_use]
    pub fn is_stereo_mvhevc_encode_supported() -> bool {
        ffi::dynamic::VTIsStereoMVHEVCEncodeSupported()
            .is_ok_and(|is_supported| unsafe { is_supported() } != 0)
    }

    /// Returns the current source-pixel-buffer pool, retaining it so the
    /// returned wrapper owns its lifetime independently of the session.
    #[must_use]
    pub fn pixel_buffer_pool(&self) -> Option<CVPixelBufferPool> {
        // SAFETY: `VTCompressionSessionGetPixelBufferPool` returns a borrowed reference
        // to the pool (not retained). We call CFRetain to extend its lifetime.
        let pool = unsafe { ffi::VTCompressionSessionGetPixelBufferPool(self.session) };
        if pool.is_null() {
            return None;
        }
        unsafe { CVPixelBufferPool::from_raw_borrowed(pool.cast()) }
    }

    /// Copy one `VTSession` property from the encoder.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] if the underlying property query fails.
    ///
    /// # Safety
    ///
    /// `key` must be a valid CoreFoundation string pointer accepted by
    /// `VTSessionCopyProperty` for a compression session.
    pub unsafe fn copy_property(&self, key: ffi::CFStringRef) -> Result<Option<CFType>, VTError> {
        session::copy_property(self.session.cast(), key)
    }

    /// Copy the encoder's supported-property dictionary.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] if the query fails.
    pub fn supported_property_dictionary(&self) -> Result<CFDictionary, VTError> {
        // SAFETY: `copy_supported_property_dictionary` is called with a valid session pointer.
        unsafe { session::copy_supported_property_dictionary(self.session.cast()) }
    }

    /// Copy the encoder's serializable property dictionary.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] if the query fails.
    pub fn serializable_properties(&self) -> Result<CFDictionary, VTError> {
        // SAFETY: `copy_serializable_properties` is called with a valid session pointer.
        unsafe { session::copy_serializable_properties(self.session.cast()) }
    }

    /// Set multiple encoder properties at once.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] if `VideoToolbox` rejects the dictionary.
    pub fn set_properties(&self, properties: &CFDictionary) -> Result<(), VTError> {
        // SAFETY: `set_properties` is called with a valid session pointer and a valid dictionary.
        unsafe { session::set_properties(self.session.cast(), properties) }
    }

    /// Attach a [`MultiPassStorage`] object so the session can do multi-pass
    /// encoding.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SetPropertyFailed`] if `VideoToolbox` rejects the
    /// storage object.
    pub fn set_multi_pass_storage(&self, storage: &MultiPassStorage) -> Result<(), VTError> {
        unsafe {
            self.set_property(
                ffi::kVTCompressionPropertyKey_MultiPassStorage,
                storage.as_ptr().cast(),
            )
        }
    }

    /// Begin a multi-pass encoding pass.
    ///
    /// Set `final_pass` when you know this must be the last pass.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] on a non-zero `OSStatus`.
    pub fn begin_pass(&self, final_pass: bool) -> Result<(), VTError> {
        let flags = if final_pass {
            ffi::kVTCompressionSessionBeginFinalPass
        } else {
            0
        };
        let status =
            unsafe { ffi::VTCompressionSessionBeginPass(self.session, flags, ptr::null_mut()) };
        if status == 0 {
            Ok(())
        } else {
            Err(VTError::ApiFailed {
                api: "VTCompressionSessionBeginPass",
                status,
            })
        }
    }

    /// End the current multi-pass encoding pass.
    ///
    /// Returns `true` when the encoder requests another pass.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] on a non-zero `OSStatus`.
    pub fn end_pass(&self) -> Result<bool, VTError> {
        let mut further_passes_requested: ffi::Boolean = 0;
        let status = unsafe {
            ffi::VTCompressionSessionEndPass(
                self.session,
                (&raw mut further_passes_requested).cast(),
                ptr::null_mut(),
            )
        };
        if status == 0 {
            Ok(further_passes_requested != 0)
        } else {
            Err(VTError::ApiFailed {
                api: "VTCompressionSessionEndPass",
                status,
            })
        }
    }

    /// Return the time ranges the encoder wants on the next multi-pass pass.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] on a non-zero `OSStatus`.
    pub fn time_ranges_for_next_pass(&self) -> Result<Vec<ffi::CMTimeRange>, VTError> {
        let mut count: ffi::CMItemCount = 0;
        let mut ranges: *const ffi::CMTimeRange = ptr::null();
        let status = unsafe {
            ffi::VTCompressionSessionGetTimeRangesForNextPass(
                self.session,
                &raw mut count,
                &raw mut ranges,
            )
        };
        if status != 0 {
            return Err(VTError::ApiFailed {
                api: "VTCompressionSessionGetTimeRangesForNextPass",
                status,
            });
        }
        if count <= 0 || ranges.is_null() {
            return Ok(Vec::new());
        }
        let count = usize::try_from(count).map_err(|_| {
            VTError::InvalidArgument("time-range count overflowed usize".to_string())
        })?;
        Ok(unsafe { std::slice::from_raw_parts(ranges, count) }.to_vec())
    }

    fn new_internal(b: &CompressionSessionBuilder) -> Result<Self, VTError> {
        let encoder_specification = b.encoder_specification();
        let mut session_ptr: ffi::VTCompressionSessionRef = ptr::null_mut();
        // SAFETY: `VTCompressionSessionCreate` is a standard Apple SDK function.
        // All arguments are valid pointers/values: allocator is default, callback is a valid
        // C function, and session_ptr is uninitialized but properly initialized on return.
        let status = unsafe {
            ffi::VTCompressionSessionCreate(
                ffi::kCFAllocatorDefault,
                b.width,
                b.height,
                b.codec.as_cm_codec_type(),
                encoder_specification
                    .as_ref()
                    .map_or(ptr::null(), |dict| dict.as_ptr().cast_const().cast()),
                b.source_pixel_buffer_attributes
                    .as_ref()
                    .map_or(ptr::null(), |dict| dict.as_ptr().cast_const().cast()),
                ffi::kCFAllocatorDefault,
                Some(encode_callback),
                ptr::null_mut(),
                &raw mut session_ptr,
            )
        };
        if status != 0 || session_ptr.is_null() {
            return Err(VTError::SessionCreateFailed(status));
        }

        let session = Self {
            session: session_ptr,
        };

        // Apply properties.
        if let Some(rt) = b.real_time {
            session.set_property_bool(
                unsafe { ffi::kVTCompressionPropertyKey_RealTime },
                "RealTime",
                rt,
            )?;
        }
        if let Some(allow) = b.allow_frame_reordering {
            session.set_property_bool(
                unsafe { ffi::kVTCompressionPropertyKey_AllowFrameReordering },
                "AllowFrameReordering",
                allow,
            )?;
        }
        if let Some(bps) = b.average_bit_rate {
            session.set_property_i32(
                unsafe { ffi::kVTCompressionPropertyKey_AverageBitRate },
                "AverageBitRate",
                bps,
            )?;
        }
        if let Some(fps) = b.expected_frame_rate {
            session.set_property_f64(
                unsafe { ffi::kVTCompressionPropertyKey_ExpectedFrameRate },
                "ExpectedFrameRate",
                fps,
            )?;
        }
        if let Some(n) = b.max_keyframe_interval {
            session.set_property_i32(
                unsafe { ffi::kVTCompressionPropertyKey_MaxKeyFrameInterval },
                "MaxKeyFrameInterval",
                n,
            )?;
        }
        if let Some(q) = b.quality {
            session.set_property_f64(
                unsafe { ffi::kVTCompressionPropertyKey_Quality },
                "Quality",
                f64::from(q.clamp(0.0, 1.0)),
            )?;
        }
        if let Some(profile) = b.profile_level {
            session.set_property_cf_string(
                unsafe { ffi::kVTCompressionPropertyKey_ProfileLevel },
                "ProfileLevel",
                profile.as_cf_string(),
            )?;
        }

        let status = unsafe { ffi::VTCompressionSessionPrepareToEncodeFrames(session.session) };
        if status != 0 {
            return Err(VTError::PrepareFailed(status));
        }

        Ok(session)
    }

    /// Submit one frame for encoding and block until the encoder has emitted it.
    ///
    /// `presentation_time` is the frame's timestamp, e.g. `CMTime::new(0, 30)`
    /// for the first frame of a 30 fps stream and `CMTime::new(1, 30)` for the
    /// second.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::PixelBufferCreateFailed`] if the `IOSurface` can't be
    /// wrapped, [`VTError::EncodeFailed`] if the encoder rejects the frame, or
    /// [`VTError::EncoderCallback`] if the encoder reports a non-zero status
    /// asynchronously.
    pub fn encode(
        &self,
        surface: &IOSurface,
        presentation_time: CMTime,
    ) -> Result<EncodedFrame, VTError> {
        self.encode_with_properties(surface, presentation_time, FrameProperties::new())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn encode_with_properties(
        &self,
        surface: &IOSurface,
        presentation_time: CMTime,
        properties: FrameProperties,
    ) -> Result<EncodedFrame, VTError> {
        let pixel_buffer = self.wrap_iosurface(surface)?;
        let frame_properties = properties.to_dictionary();
        let slot = FrameSlot::submit(|ref_con| unsafe {
            ffi::VTCompressionSessionEncodeFrame(
                self.session,
                pixel_buffer.as_ptr().cast(),
                presentation_time,
                CMTime::INVALID,
                frame_properties
                    .as_ref()
                    .map_or(ptr::null(), |dict| dict.as_ptr().cast_const().cast()),
                ref_con,
                ptr::null_mut(),
            )
        })?;
        self.complete_frame(&slot, presentation_time)
    }

    /// Submit `image_buffer` for encoding and await the encoded `CMSampleBuffer`.
    ///
    /// The frame is submitted when this is called. If the encoder is still
    /// holding it back (for example for frame reordering) when the future is
    /// first polled, the poll forces it out with
    /// `VTCompressionSessionCompleteFrames` up to `presentation_timestamp`,
    /// blocking the polling thread until the encoder emits it.
    ///
    /// This method requires the crate's `async` feature.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] if the frame submission is rejected,
    /// [`VTError::CompleteFailed`] if forcing the output fails, or
    /// [`VTError::EncoderCallback`] if the encoder callback reports a failure or
    /// drops the frame without a `CMSampleBuffer`.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn encode_frame_async(
        &self,
        image_buffer: CVPixelBuffer,
        presentation_timestamp: CMTime,
        duration: CMTime,
        frame_properties: Option<CFDictionary>,
    ) -> impl Future<Output = Result<CMSampleBuffer, VTError>> + '_ {
        let submitted = FrameSlot::submit(|ref_con| unsafe {
            ffi::VTCompressionSessionEncodeFrame(
                self.session,
                image_buffer.as_ptr().cast(),
                presentation_timestamp,
                duration,
                frame_properties
                    .as_ref()
                    .map_or(ptr::null(), |dict| dict.as_ptr().cast_const().cast()),
                ref_con,
                ptr::null_mut(),
            )
        });
        let mut forced = false;
        core::future::poll_fn(move |cx| {
            let slot = match &submitted {
                Ok(slot) => slot,
                Err(error) => return Poll::Ready(Err(error.clone())),
            };
            if let Some(output) = slot.poll_output(cx.waker()) {
                return Poll::Ready(sample_buffer_from(output));
            }
            if !forced {
                forced = true;
                let status = unsafe {
                    ffi::VTCompressionSessionCompleteFrames(self.session, presentation_timestamp)
                };
                if status != 0 {
                    return Poll::Ready(Err(VTError::CompleteFailed(status)));
                }
                if let Some(output) = slot.poll_output(cx.waker()) {
                    return Poll::Ready(sample_buffer_from(output));
                }
            }
            Poll::Pending
        })
    }

    /// Complete pending output, invalidate the encoder, and release its native resources.
    ///
    /// Unlike `Drop`, this reports a failure to complete the pending frames.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::CompleteFailed`] if
    /// `VTCompressionSessionCompleteFrames` fails.
    pub fn invalidate(mut self) -> Result<(), VTError> {
        self.teardown()
    }

    /// Submit one multi-image frame (for example stereo MV-HEVC left/right eye
    /// images) and block until the encoder has emitted it.
    ///
    /// The `tagged_buffer_group` is a `CoreMedia` `CMTaggedBufferGroup` containing
    /// the images that make up one logical frame.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::Unsupported`] before macOS 14.0,
    /// [`VTError::EncodeFailed`] if the encoder rejects the frame or
    /// [`VTError::EncoderCallback`] if the completion callback reports failure.
    pub fn encode_multi_image(
        &self,
        tagged_buffer_group: &TaggedBufferGroup,
        presentation_time: CMTime,
    ) -> Result<EncodedFrame, VTError> {
        let encode = ffi::dynamic::VTCompressionSessionEncodeMultiImageFrame()?;
        let slot = FrameSlot::submit(|ref_con| unsafe {
            encode(
                self.session,
                tagged_buffer_group.as_ptr(),
                presentation_time,
                CMTime::INVALID,
                ptr::null(),
                ref_con,
                ptr::null_mut(),
            )
        })?;
        self.complete_frame(&slot, presentation_time)
    }

    fn complete_frame(
        &self,
        slot: &FrameSlot,
        presentation_time: CMTime,
    ) -> Result<EncodedFrame, VTError> {
        let output = match slot.take() {
            Some(output) => output,
            None => {
                let status = unsafe {
                    ffi::VTCompressionSessionCompleteFrames(self.session, presentation_time)
                };
                if status != 0 {
                    return Err(VTError::CompleteFailed(status));
                }
                slot.wait()
            }
        };
        EncodedFrame::from_output(output?, presentation_time)
    }

    #[allow(clippy::unused_self)]
    fn wrap_iosurface(&self, surface: &IOSurface) -> Result<CVPixelBuffer, VTError> {
        let mut pb: ffi::CVPixelBufferRef = ptr::null_mut();
        let status = unsafe {
            ffi::CVPixelBufferCreateWithIOSurface(
                ffi::kCFAllocatorDefault,
                surface.as_ptr().cast::<c_void>(),
                ptr::null(),
                &raw mut pb,
            )
        };
        if status != 0 || pb.is_null() {
            return Err(VTError::PixelBufferCreateFailed(status));
        }
        unsafe { CVPixelBuffer::from_raw(pb.cast()) }
            .ok_or(VTError::PixelBufferCreateFailed(status))
    }

    fn set_property_bool(
        &self,
        key: ffi::CFStringRef,
        key_name: &'static str,
        value: bool,
    ) -> Result<(), VTError> {
        let cf_value = if value {
            unsafe { ffi::kCFBooleanTrue }
        } else {
            unsafe { ffi::kCFBooleanFalse }
        };
        let status = unsafe { ffi::VTSessionSetProperty(self.session, key, cf_value.cast()) };
        if status != 0 {
            return Err(VTError::SetPropertyFailed {
                key: key_name.to_string(),
                status,
            });
        }
        Ok(())
    }

    fn set_property_i32(
        &self,
        key: ffi::CFStringRef,
        key_name: &'static str,
        value: i32,
    ) -> Result<(), VTError> {
        let value_ref = unsafe {
            ffi::CFNumberCreate(
                ffi::kCFAllocatorDefault,
                ffi::kCFNumberSInt32Type,
                core::ptr::from_ref(&value).cast(),
            )
        };
        let status = unsafe { ffi::VTSessionSetProperty(self.session, key, value_ref.cast()) };
        unsafe { ffi::CFRelease(value_ref.cast()) };
        if status != 0 {
            return Err(VTError::SetPropertyFailed {
                key: key_name.to_string(),
                status,
            });
        }
        Ok(())
    }

    fn set_property_f64(
        &self,
        key: ffi::CFStringRef,
        key_name: &'static str,
        value: f64,
    ) -> Result<(), VTError> {
        let value_ref = unsafe {
            ffi::CFNumberCreate(
                ffi::kCFAllocatorDefault,
                ffi::kCFNumberFloat64Type,
                core::ptr::from_ref(&value).cast(),
            )
        };
        let status = unsafe { ffi::VTSessionSetProperty(self.session, key, value_ref.cast()) };
        unsafe { ffi::CFRelease(value_ref.cast()) };
        if status != 0 {
            return Err(VTError::SetPropertyFailed {
                key: key_name.to_string(),
                status,
            });
        }
        Ok(())
    }

    fn set_property_cf_string(
        &self,
        key: ffi::CFStringRef,
        key_name: &'static str,
        value: ffi::CFStringRef,
    ) -> Result<(), VTError> {
        let status = unsafe { ffi::VTSessionSetProperty(self.session, key, value.cast()) };
        if status != 0 {
            return Err(VTError::SetPropertyFailed {
                key: key_name.to_string(),
                status,
            });
        }
        Ok(())
    }

    /// Set an arbitrary property on the underlying `VTCompressionSession`.
    /// `value` must be a CoreFoundation object (`CFNumberRef`,
    /// `CFBooleanRef`, `CFStringRef`, ...).
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SetPropertyFailed`] if Apple rejects the
    /// key/value pair.
    ///
    /// # Safety
    ///
    /// `key` must be a valid `CFStringRef`; `value` must be a valid
    /// CoreFoundation object pointer for the property's expected type
    /// (per Apple's `VTCompressionProperties.h`).
    pub unsafe fn set_property(
        &self,
        key: ffi::CFStringRef,
        value: ffi::CFTypeRef,
    ) -> Result<(), VTError> {
        let status = ffi::VTSessionSetProperty(self.session, key, value);
        if status != 0 {
            return Err(VTError::SetPropertyFailed {
                key: "<custom>".to_string(),
                status,
            });
        }
        Ok(())
    }

    fn teardown(&mut self) -> Result<(), VTError> {
        if self.session.is_null() {
            return Ok(());
        }

        let complete_status =
            unsafe { ffi::VTCompressionSessionCompleteFrames(self.session, ffi::CMTime::INVALID) };
        unsafe {
            ffi::VTCompressionSessionInvalidate(self.session);
            ffi::CFRelease(self.session.cast());
        }
        self.session = ptr::null_mut();

        if complete_status == 0 {
            Ok(())
        } else {
            Err(VTError::CompleteFailed(complete_status))
        }
    }
}

impl Drop for CompressionSession {
    fn drop(&mut self) {
        let _ = self.teardown();
    }
}

impl core::fmt::Debug for CompressionSession {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CompressionSession")
            .field("session", &self.session)
            .finish_non_exhaustive()
    }
}

// ---- internal callback ----

#[cfg(feature = "async")]
fn sample_buffer_from(output: Result<FrameOutput, VTError>) -> Result<CMSampleBuffer, VTError> {
    output?.sample_buffer.ok_or(VTError::EncoderCallback(-1))
}

unsafe extern "C" fn encode_callback(
    _output_callback_ref_con: *mut c_void,
    source_frame_ref_con: *mut c_void,
    status: ffi::OSStatus,
    info_flags: ffi::VTEncodeInfoFlags,
    sample_buffer: ffi::CMSampleBufferRef,
) {
    catch_user_panic("videotoolbox::compression::encode_callback", || {
        let Some(slot) = (unsafe { FrameSlot::release_native(source_frame_ref_con) }) else {
            return;
        };
        let result = if status == 0 {
            Ok(FrameOutput {
                info_flags,
                sample_buffer: unsafe { CMSampleBuffer::from_raw_borrowed(sample_buffer.cast()) },
            })
        } else {
            Err(VTError::EncoderCallback(status))
        };
        slot.complete(result);
    });
}

#[cfg(test)]
mod tests {
    use core::ffi::c_void;
    use core::ptr;
    use std::sync::Arc;

    use apple_cf::cf::{CFNumber, CFString, CFType};
    use apple_cf::cm::CMTime;

    use super::{
        encode_callback, CompressionSessionBuilder, EncodedFrame, EncoderGpu, FrameProperties,
        FrameSlot, HardwareAcceleration, ProfileLevel,
    };
    use crate::{error::VTError, ffi, session::Codec};

    fn key(raw: ffi::CFStringRef) -> CFType {
        unsafe { CFType::from_raw_borrowed(raw.cast_mut().cast()) }.expect("SDK key")
    }

    #[test]
    fn builder_new_starts_with_expected_defaults() {
        let builder = CompressionSessionBuilder::new(1920, 1080, Codec::H264);

        assert_eq!(builder.width, 1920);
        assert_eq!(builder.height, 1080);
        assert_eq!(builder.codec, Codec::H264);
        assert_eq!(builder.real_time, None);
        assert_eq!(builder.allow_frame_reordering, None);
        assert_eq!(builder.average_bit_rate, None);
        assert_eq!(builder.expected_frame_rate, None);
        assert_eq!(builder.max_keyframe_interval, None);
        assert_eq!(builder.quality, None);
        assert_eq!(builder.profile_level, None);
        assert_eq!(builder.hardware_acceleration, None);
        assert_eq!(builder.encoder_id, None);
        assert_eq!(builder.low_latency_rate_control, None);
        assert_eq!(builder.encoder_gpu, None);
        assert!(builder.source_pixel_buffer_attributes.is_none());
        assert!(builder.encoder_specification().is_none());
    }

    #[test]
    fn builder_chain_records_requested_settings() {
        let builder = CompressionSessionBuilder::new(3840, 2160, Codec::HEVC)
            .with_real_time(true)
            .with_allow_frame_reordering(false)
            .with_average_bit_rate(24_000_000)
            .with_expected_frame_rate(59.94)
            .with_max_keyframe_interval(120)
            .with_quality(0.75)
            .with_profile_level(ProfileLevel::HEVCMain10AutoLevel);

        assert_eq!(builder.width, 3840);
        assert_eq!(builder.height, 2160);
        assert_eq!(builder.codec, Codec::HEVC);
        assert_eq!(builder.real_time, Some(true));
        assert_eq!(builder.allow_frame_reordering, Some(false));
        assert_eq!(builder.average_bit_rate, Some(24_000_000));
        assert!(
            (builder
                .expected_frame_rate
                .expect("expected frame rate should be set")
                - 59.94)
                .abs()
                < 1.0e-9
        );
        assert_eq!(builder.max_keyframe_interval, Some(120));
        assert!((builder.quality.expect("quality should be set") - 0.75).abs() < f32::EPSILON);
        assert_eq!(
            builder.profile_level,
            Some(ProfileLevel::HEVCMain10AutoLevel)
        );
    }

    #[test]
    fn encoder_specification_carries_every_requested_key() {
        let builder = CompressionSessionBuilder::new(64, 64, Codec::H264)
            .with_hardware_acceleration(HardwareAcceleration::Required)
            .with_encoder_id("com.example.encoder")
            .with_low_latency_rate_control(true)
            .with_encoder_gpu(EncoderGpu::Required(u64::MAX));

        let spec = builder
            .encoder_specification()
            .expect("specification should be built");

        assert_eq!(spec.len(), 4);
        let require = spec
            .get(&key(unsafe {
                ffi::kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder
            }))
            .expect("require key");
        assert_eq!(require.as_ptr().cast_const(), super::cf_boolean(true));
        let encoder_id = spec
            .get(&key(unsafe { ffi::kVTVideoEncoderSpecification_EncoderID }))
            .expect("encoder id key");
        assert_eq!(encoder_id.type_id(), CFString::type_id());
        let encoder_id =
            unsafe { CFString::from_raw_borrowed(encoder_id.as_ptr()) }.expect("string");
        assert_eq!(encoder_id.to_string(), "com.example.encoder");
        let low_latency = spec
            .get(&key(unsafe {
                ffi::kVTVideoEncoderSpecification_EnableLowLatencyRateControl
            }))
            .expect("low-latency key");
        assert_eq!(low_latency.as_ptr().cast_const(), super::cf_boolean(true));
        let registry_id = spec
            .get(&key(unsafe {
                ffi::kVTVideoEncoderSpecification_RequiredEncoderGPURegistryID
            }))
            .expect("registry id key");
        assert_eq!(registry_id.type_id(), CFNumber::type_id());
        let registry_id =
            unsafe { CFNumber::from_raw_borrowed(registry_id.as_ptr()) }.expect("number");
        assert_eq!(registry_id.to_u64(), Some(u64::MAX));
    }

    #[test]
    fn encoder_specification_maps_preferred_and_disabled_acceleration() {
        let preferred = CompressionSessionBuilder::new(64, 64, Codec::H264)
            .with_hardware_acceleration(HardwareAcceleration::Preferred)
            .with_encoder_gpu(EncoderGpu::Preferred(7))
            .encoder_specification()
            .expect("specification should be built");
        let enable =
            key(unsafe { ffi::kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder });
        assert_eq!(
            preferred
                .get(&enable)
                .map(|value| value.as_ptr().cast_const()),
            Some(super::cf_boolean(true))
        );
        assert!(preferred
            .get(&key(unsafe {
                ffi::kVTVideoEncoderSpecification_PreferredEncoderGPURegistryID
            }))
            .is_some());

        let disabled = CompressionSessionBuilder::new(64, 64, Codec::H264)
            .with_hardware_acceleration(HardwareAcceleration::Disabled)
            .encoder_specification()
            .expect("specification should be built");
        assert_eq!(disabled.len(), 1);
        assert_eq!(
            disabled
                .get(&enable)
                .map(|value| value.as_ptr().cast_const()),
            Some(super::cf_boolean(false))
        );
    }

    #[test]
    fn frame_properties_only_build_a_dictionary_when_needed() {
        assert!(FrameProperties::new().to_dictionary().is_none());
        assert_eq!(FrameProperties::default(), FrameProperties::new());

        let dictionary = FrameProperties::new()
            .with_force_key_frame(true)
            .to_dictionary()
            .expect("forced keyframe needs a dictionary");
        assert_eq!(dictionary.len(), 1);
        let force = dictionary
            .get(&key(unsafe { ffi::kVTEncodeFrameOptionKey_ForceKeyFrame }))
            .expect("force keyframe key");
        assert_eq!(force.as_ptr().cast_const(), super::cf_boolean(true));
    }

    #[test]
    fn build_rejects_non_positive_dimensions_before_entering_ffi() {
        let error = CompressionSessionBuilder::new(0, 1080, Codec::H264)
            .build()
            .expect_err("zero width must fail before session creation");

        assert_eq!(
            error,
            VTError::InvalidArgument("width/height must be positive (got 0x1080)".to_owned())
        );
    }

    #[test]
    fn output_callback_completes_the_frame_it_belongs_to() {
        let first = FrameSlot::new();
        let second = FrameSlot::new();
        let first_ref_con = first.native_ref_con();
        let second_ref_con = second.native_ref_con();

        unsafe {
            encode_callback(ptr::null_mut(), second_ref_con, 0, 7, ptr::null_mut());
            encode_callback(ptr::null_mut(), first_ref_con, -12902, 0, ptr::null_mut());
        }

        assert_eq!(second.wait().map(|output| output.info_flags), Ok(7));
        assert_eq!(
            first.wait().map(|output| output.info_flags),
            Err(VTError::EncoderCallback(-12902))
        );
        assert_eq!(Arc::strong_count(&first), 1);
        assert_eq!(Arc::strong_count(&second), 1);
    }

    #[test]
    fn duplicate_output_callback_is_ignored() {
        let slot = FrameSlot::new();
        let ref_con = slot.native_ref_con();

        unsafe {
            encode_callback(ptr::null_mut(), ref_con, 0, 1, ptr::null_mut());
            encode_callback(ptr::null_mut(), ref_con, -1, 2, ptr::null_mut());
            encode_callback(ptr::null_mut(), ptr::null_mut(), -1, 3, ptr::null_mut());
        }

        assert_eq!(slot.wait().map(|output| output.info_flags), Ok(1));
        assert_eq!(Arc::strong_count(&slot), 1);
    }

    fn observe(ref_con: *mut c_void) -> std::sync::Weak<FrameSlot> {
        let native = unsafe { Arc::from_raw(ref_con.cast::<FrameSlot>().cast_const()) };
        let weak = Arc::downgrade(&native);
        let _ = Arc::into_raw(native);
        weak
    }

    #[test]
    fn rejected_submission_releases_the_native_reference() {
        let mut weak = None;

        let result = FrameSlot::submit(|ref_con| {
            weak = Some(observe(ref_con));
            -12902
        });

        assert_eq!(result.err(), Some(VTError::EncodeFailed(-12902)));
        assert!(weak.expect("submit ran").upgrade().is_none());
    }

    #[test]
    fn rejected_submission_after_a_synchronous_callback_releases_once() {
        let mut weak = None;

        let result = FrameSlot::submit(|ref_con| {
            weak = Some(observe(ref_con));
            unsafe { encode_callback(ptr::null_mut(), ref_con, 0, 0, ptr::null_mut()) };
            -12902
        });

        assert_eq!(result.err(), Some(VTError::EncodeFailed(-12902)));
        assert!(weak.expect("submit ran").upgrade().is_none());
    }

    #[test]
    fn accepted_submission_hands_one_reference_to_the_encoder() {
        let slot = FrameSlot::submit(|_| 0).expect("submission accepted");

        assert_eq!(Arc::strong_count(&slot), 2);
        let ref_con = Arc::as_ptr(&slot).cast_mut().cast::<c_void>();
        unsafe { encode_callback(ptr::null_mut(), ref_con, 0, 5, ptr::null_mut()) };
        assert_eq!(Arc::strong_count(&slot), 1);
        assert_eq!(
            slot.take().map(|output| output.map(|o| o.info_flags)),
            Some(Ok(5))
        );
    }

    #[test]
    fn dropped_frame_keeps_its_source_timestamp() {
        let frame = EncodedFrame::from_output(
            super::FrameOutput {
                info_flags: ffi::kVTEncodeInfo_FrameDropped,
                sample_buffer: None,
            },
            CMTime::new(9, 30),
        )
        .expect("dropped frame is not an error");

        assert!(frame.data.is_empty());
        assert_eq!(frame.presentation_time, CMTime::new(9, 30));
        assert_eq!(frame.info_flags, ffi::kVTEncodeInfo_FrameDropped);
        assert!(frame.cm_sample_buffer().is_none());
    }

    #[test]
    fn encoded_frame_accessors_handle_missing_sample_buffer() {
        let frame = EncodedFrame {
            data: vec![1, 2, 3],
            presentation_time: CMTime::new(10, 30),
            info_flags: 7,
            sample_buffer: None,
        };
        let cloned = frame.clone();
        let debug = format!("{cloned:?}");

        assert!(frame.cm_sample_buffer().is_none());
        assert!(frame.cm_sample_buffer_ptr().is_null());
        assert_eq!(cloned.data, vec![1, 2, 3]);
        assert_eq!(cloned.presentation_time, CMTime::new(10, 30));
        assert_eq!(cloned.info_flags, 7);
        assert!(debug.contains("EncodedFrame"));
        assert!(debug.contains("presentation_time"));
    }

    #[test]
    fn profile_level_maps_to_expected_cfstring_constants() {
        assert_eq!(ProfileLevel::H264HighAutoLevel.as_cf_string(), unsafe {
            ffi::kVTProfileLevel_H264_High_AutoLevel
        });
        assert_eq!(ProfileLevel::HEVCMain10AutoLevel.as_cf_string(), unsafe {
            ffi::kVTProfileLevel_HEVC_Main10_AutoLevel
        });
    }

    #[test]
    fn compression_property_keys_are_non_null_and_distinct() {
        let real_time = unsafe { ffi::kVTCompressionPropertyKey_RealTime };
        let profile_level = unsafe { ffi::kVTCompressionPropertyKey_ProfileLevel };
        let quality = unsafe { ffi::kVTCompressionPropertyKey_Quality };

        assert!(!real_time.is_null());
        assert!(!profile_level.is_null());
        assert!(!quality.is_null());
        assert_ne!(real_time, profile_level);
        assert_ne!(real_time, quality);
        assert_ne!(profile_level, quality);
    }

    #[test]
    fn profile_level_round_trips_through_its_name() {
        for profile in ProfileLevel::ALL {
            assert_eq!(ProfileLevel::from_name(profile.name()), Some(*profile));
        }
    }

    #[test]
    fn profile_level_lookup_ignores_case_and_separators() {
        let expected = Some(ProfileLevel::H264HighAutoLevel);
        assert_eq!(ProfileLevel::from_name("h264highautolevel"), expected);
        assert_eq!(ProfileLevel::from_name("H264_High_AutoLevel"), expected);
        assert_eq!(ProfileLevel::from_name("h264 high auto level"), expected);
        assert_eq!(ProfileLevel::from_name("nope"), None);
    }
}
