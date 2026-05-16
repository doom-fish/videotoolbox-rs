//! [`CompressionSession`] — hardware H.264/HEVC/ProRes encoder.

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use apple_cf::iosurface::IOSurface;

use crate::error::VTError;
use crate::ffi;
use crate::session::Codec;

/// One encoded frame produced by [`CompressionSession::encode`].
///
/// Wraps the encoder's `CMSampleBuffer` as a safe [`apple_cf::cm::CMSampleBuffer`]
/// so downstream crates (e.g. `avassetwriter-rs`) can hand it off zero-copy
/// without dealing with raw `*mut c_void` pointers.
pub struct EncodedFrame {
    /// Encoded bitstream bytes (NAL units for H.264/HEVC, frame data for `ProRes`).
    pub data: Vec<u8>,
    /// Presentation timestamp of the source frame (numerator, timescale).
    pub presentation_time: (i64, i32),
    /// Encoder hint flags (e.g. dropped, asynchronous).
    pub info_flags: u32,
    /// Underlying `CoreMedia` sample buffer. `None` for dropped frames.
    sample_buffer: Option<apple_cf::cm::CMSampleBuffer>,
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
    pub fn cm_sample_buffer_ptr(&self) -> *mut core::ffi::c_void {
        self.sample_buffer
            .as_ref()
            .map_or(core::ptr::null_mut(), apple_cf::cm::CMSampleBuffer::as_ptr)
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
}

/// Encoded profile/level for the underlying codec. Maps to
/// `kVTProfileLevel_*` `CFStringRef` constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProfileLevel {
    H264BaselineAutoLevel,
    H264MainAutoLevel,
    H264HighAutoLevel,
    HEVCMainAutoLevel,
    HEVCMain10AutoLevel,
}

impl ProfileLevel {
    pub(crate) fn as_cf_string(self) -> ffi::CFStringRef {
        unsafe {
            match self {
                Self::H264BaselineAutoLevel => ffi::kVTProfileLevel_H264_Baseline_AutoLevel,
                Self::H264MainAutoLevel => ffi::kVTProfileLevel_H264_Main_AutoLevel,
                Self::H264HighAutoLevel => ffi::kVTProfileLevel_H264_High_AutoLevel,
                Self::HEVCMainAutoLevel => ffi::kVTProfileLevel_HEVC_Main_AutoLevel,
                Self::HEVCMain10AutoLevel => ffi::kVTProfileLevel_HEVC_Main10_AutoLevel,
            }
        }
    }
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
}

/// Hardware-accelerated video compression session.
///
/// Construct via [`CompressionSession::builder`]. Each session owns a Swift-side
/// `VTCompressionSessionRef`; the underlying encoder is invalidated on drop.
pub struct CompressionSession {
    session: ffi::VTCompressionSessionRef,
    state: Arc<EncoderState>,
}

// SAFETY: VideoToolbox sessions are documented as thread-safe for concurrent
// `encode` calls; the underlying encoder owns its own dispatch queue.
unsafe impl Send for CompressionSession {}
unsafe impl Sync for CompressionSession {}

struct EncoderState {
    out_tx: Mutex<mpsc::Sender<Result<EncodedFrame, VTError>>>,
    out_rx: Mutex<mpsc::Receiver<Result<EncodedFrame, VTError>>>,
}

impl CompressionSession {
    /// Convenience: start a builder.
    #[must_use]
    pub const fn builder(width: i32, height: i32, codec: Codec) -> CompressionSessionBuilder {
        CompressionSessionBuilder::new(width, height, codec)
    }

    fn new_internal(b: &CompressionSessionBuilder) -> Result<Self, VTError> {
        let (tx, rx) = mpsc::channel();
        let state = Arc::new(EncoderState {
            out_tx: Mutex::new(tx),
            out_rx: Mutex::new(rx),
        });
        let state_for_callback = state.clone();
        let callback_ref_con = Arc::into_raw(state_for_callback)
            .cast::<c_void>()
            .cast_mut();

        let mut session_ptr: ffi::VTCompressionSessionRef = ptr::null_mut();
        let status = unsafe {
            ffi::VTCompressionSessionCreate(
                ffi::kCFAllocatorDefault,
                b.width,
                b.height,
                b.codec.as_cm_codec_type(),
                ptr::null(),
                ptr::null(),
                ffi::kCFAllocatorDefault,
                Some(encode_callback),
                callback_ref_con,
                &mut session_ptr,
            )
        };
        if status != 0 || session_ptr.is_null() {
            // Drop the leaked Arc clone since the encoder will never call us back.
            unsafe { drop(Arc::from_raw(callback_ref_con.cast::<EncoderState>())) };
            return Err(VTError::SessionCreateFailed(status));
        }

        let session = Self {
            session: session_ptr,
            state,
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
    /// `presentation_time` is `(value, timescale)`, e.g. `(0, 30)` for the first
    /// frame of a 30 fps stream and `(1, 30)` for the second.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::PixelBufferCreateFailed`] if the `IOSurface` can't be
    /// wrapped, [`VTError::EncodeFailed`] if the encoder rejects the frame, or
    /// [`VTError::EncoderCallback`] if the encoder reports a non-zero status
    /// asynchronously.
    ///
    /// # Panics
    ///
    /// Panics if the encoder's internal callback receiver mutex is poisoned
    /// (only possible if a previous callback panicked while holding it).
    pub fn encode(
        &self,
        surface: &IOSurface,
        presentation_time: (i64, i32),
    ) -> Result<EncodedFrame, VTError> {
        let pixel_buffer = self.wrap_iosurface(surface)?;

        let pts = ffi::CMTime::new(presentation_time.0, presentation_time.1);
        let status = unsafe {
            ffi::VTCompressionSessionEncodeFrame(
                self.session,
                pixel_buffer,
                pts,
                ffi::CMTime::INVALID,
                ptr::null(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        unsafe { ffi::CFRelease(pixel_buffer.cast()) };
        if status != 0 {
            return Err(VTError::EncodeFailed(status));
        }

        let status =
            unsafe { ffi::VTCompressionSessionCompleteFrames(self.session, ffi::CMTime::INVALID) };
        if status != 0 {
            return Err(VTError::CompleteFailed(status));
        }

        let rx = self.state.out_rx.lock().expect("encoder rx mutex poisoned");
        rx.recv().map_err(|_| VTError::EncoderCallback(-1))?
    }

    #[allow(clippy::unused_self)]
    fn wrap_iosurface(&self, surface: &IOSurface) -> Result<ffi::CVPixelBufferRef, VTError> {
        let mut pb: ffi::CVPixelBufferRef = ptr::null_mut();
        let status = unsafe {
            ffi::CVPixelBufferCreateWithIOSurface(
                ffi::kCFAllocatorDefault,
                surface.as_ptr().cast::<c_void>(),
                ptr::null(),
                &mut pb,
            )
        };
        if status != 0 || pb.is_null() {
            return Err(VTError::PixelBufferCreateFailed(status));
        }
        Ok(pb)
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
        let status = unsafe { ffi::VTSessionSetProperty(self.session, key, cf_value) };
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
        let status = unsafe { ffi::VTSessionSetProperty(self.session, key, value_ref) };
        unsafe { ffi::CFRelease(value_ref) };
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
        let status = unsafe { ffi::VTSessionSetProperty(self.session, key, value_ref) };
        unsafe { ffi::CFRelease(value_ref) };
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
}

impl Drop for CompressionSession {
    fn drop(&mut self) {
        if !self.session.is_null() {
            unsafe {
                ffi::VTCompressionSessionInvalidate(self.session);
                ffi::CFRelease(self.session.cast());
            }
        }
        // The Arc<EncoderState> we leaked into VTCompressionSessionCreate's
        // ref-con cannot be retrieved here cleanly without unsafe gymnastics —
        // VideoToolbox doesn't surface the pointer back. Leak it; it's tiny and
        // sessions are long-lived.
        //
        // TODO: bind to VTCompressionSessionGetRefCon if/when we can rely on it
        // being available.
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

unsafe extern "C" fn encode_callback(
    output_callback_ref_con: *mut c_void,
    _source_frame_ref_con: *mut c_void,
    status: ffi::OSStatus,
    info_flags: ffi::VTEncodeInfoFlags,
    sample_buffer: ffi::CMSampleBufferRef,
) {
    // We borrow but do not consume the Arc — the session keeps it alive for
    // the lifetime of the encoder.
    let state_ptr = output_callback_ref_con.cast::<EncoderState>();
    let state = Arc::from_raw(state_ptr);
    let state_clone = state.clone();
    core::mem::forget(state); // restore refcount

    let result = if status != 0 {
        Err(VTError::EncoderCallback(status))
    } else if sample_buffer.is_null() {
        // Frame was dropped — emit an empty frame with the dropped flag set.
        Ok(EncodedFrame {
            data: Vec::new(),
            presentation_time: (0, 0),
            info_flags,
            sample_buffer: None,
        })
    } else {
        let pts = ffi::CMSampleBufferGetPresentationTimeStamp(sample_buffer);
        let block_buffer = ffi::CMSampleBufferGetDataBuffer(sample_buffer);
        if block_buffer.is_null() {
            Err(VTError::EncoderCallback(-2))
        } else {
            let len = ffi::CMBlockBufferGetDataLength(block_buffer);
            let mut data = vec![0u8; len];
            let copy_status = ffi::CMBlockBufferCopyDataBytes(
                block_buffer,
                0,
                len,
                data.as_mut_ptr().cast::<c_void>(),
            );
            if copy_status != 0 {
                Err(VTError::EncoderCallback(copy_status))
            } else {
                // Wrap the CMSampleBuffer in a safe apple_cf type. The
                // wrapper retains-on-take so the encoder's reference is
                // unaffected.
                let safe = apple_cf::cm::CMSampleBuffer::from_raw_retained(sample_buffer);
                Ok(EncodedFrame {
                    data,
                    presentation_time: (pts.value, pts.timescale),
                    info_flags,
                    sample_buffer: safe,
                })
            }
        }
    };

    let tx = state_clone
        .out_tx
        .lock()
        .expect("encoder tx mutex poisoned");
    let _ = tx.send(result);
}

// CString import retained for future async API; suppress unused warning.
#[allow(dead_code)]
fn _retain_cstring(_: CString) {}
#[allow(dead_code)]
const fn _retain_c_char_ptr(_: *const c_char) {}
