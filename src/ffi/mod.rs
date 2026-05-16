//! Raw `extern "C"` declarations against `<VideoToolbox/VideoToolbox.h>`,
//! `<CoreMedia/CoreMedia.h>`, `<CoreVideo/CVPixelBuffer.h>`, and
//! `<CoreFoundation/CoreFoundation.h>`.
//!
//! These are intentionally `unsafe` and opaque (`*const c_void` / `*mut c_void`).
//! Safe wrappers live in [`crate::compression`] and [`crate::session`].

#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// ---- type aliases that match the C headers ----

pub type OSStatus = i32;
pub type CMVideoCodecType = u32;
pub type CMTimeFlags = u32;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CMTime {
    pub value: i64,
    pub timescale: i32,
    pub flags: CMTimeFlags,
    pub epoch: i64,
}

impl CMTime {
    pub const VALID: CMTimeFlags = 1;

    /// Construct a `CMTime` from a numerator + denominator. e.g. `CMTime::new(1, 30)`
    /// for 1/30s (one 30 fps frame).
    #[must_use]
    pub const fn new(value: i64, timescale: i32) -> Self {
        Self {
            value,
            timescale,
            flags: Self::VALID,
            epoch: 0,
        }
    }

    pub const INVALID: Self = Self {
        value: 0,
        timescale: 0,
        flags: 0,
        epoch: 0,
    };
}

// CMVideoCodecType FourCC codes (ASCII bytes).
pub const kCMVideoCodecType_H264: CMVideoCodecType = u32::from_be_bytes(*b"avc1");
pub const kCMVideoCodecType_HEVC: CMVideoCodecType = u32::from_be_bytes(*b"hvc1");
pub const kCMVideoCodecType_AppleProRes422: CMVideoCodecType = u32::from_be_bytes(*b"apcn");
pub const kCMVideoCodecType_AppleProRes422HQ: CMVideoCodecType = u32::from_be_bytes(*b"apch");
pub const kCMVideoCodecType_AppleProRes422LT: CMVideoCodecType = u32::from_be_bytes(*b"apcs");
pub const kCMVideoCodecType_AppleProRes422Proxy: CMVideoCodecType = u32::from_be_bytes(*b"apco");
pub const kCMVideoCodecType_AppleProRes4444: CMVideoCodecType = u32::from_be_bytes(*b"ap4h");

// VTEncodeInfoFlags (returned via the encode callback)
pub const kVTEncodeInfo_Asynchronous: u32 = 1;
pub const kVTEncodeInfo_FrameDropped: u32 = 1 << 1;

// ---- CoreFoundation minimum required surface ----

pub type CFAllocatorRef = *const c_void;
pub type CFTypeRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFNumberRef = *const c_void;
pub type CFBooleanRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFMutableDictionaryRef = *mut c_void;

pub type CFNumberType = c_int;
pub const kCFNumberSInt32Type: CFNumberType = 3;
pub const kCFNumberFloat64Type: CFNumberType = 13;

// CFNumberFormatter etc. — not needed for the encoder API.
pub const kCFStringEncodingUTF8: u32 = 0x0800_0100;

extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFBooleanTrue: CFBooleanRef;
    pub static kCFBooleanFalse: CFBooleanRef;
    pub static kCFTypeDictionaryKeyCallBacks: c_void;
    pub static kCFTypeDictionaryValueCallBacks: c_void;

    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;

    pub fn CFNumberCreate(
        allocator: CFAllocatorRef,
        the_type: CFNumberType,
        value_ptr: *const c_void,
    ) -> CFNumberRef;

    pub fn CFStringCreateWithCString(
        allocator: CFAllocatorRef,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;

    pub fn CFDictionaryCreateMutable(
        allocator: CFAllocatorRef,
        capacity: isize,
        key_callbacks: *const c_void,
        value_callbacks: *const c_void,
    ) -> CFMutableDictionaryRef;
    pub fn CFDictionarySetValue(
        dict: CFMutableDictionaryRef,
        key: *const c_void,
        value: *const c_void,
    );
}

// ---- CoreMedia: CMSampleBuffer ----

pub type CMSampleBufferRef = *mut c_void;
pub type CMBlockBufferRef = *mut c_void;
pub type CMFormatDescriptionRef = *mut c_void;

extern "C" {
    pub fn CMSampleBufferGetDataBuffer(sbuf: CMSampleBufferRef) -> CMBlockBufferRef;
    pub fn CMSampleBufferGetTotalSampleSize(sbuf: CMSampleBufferRef) -> usize;
    pub fn CMSampleBufferGetFormatDescription(sbuf: CMSampleBufferRef) -> CMFormatDescriptionRef;
    pub fn CMSampleBufferGetPresentationTimeStamp(sbuf: CMSampleBufferRef) -> CMTime;
    pub fn CMSampleBufferGetDuration(sbuf: CMSampleBufferRef) -> CMTime;
    pub fn CMBlockBufferCopyDataBytes(
        the_source_buffer: CMBlockBufferRef,
        offset_to_data: usize,
        data_length: usize,
        destination: *mut c_void,
    ) -> OSStatus;
    pub fn CMBlockBufferGetDataLength(the_buffer: CMBlockBufferRef) -> usize;
}

// ---- CoreVideo: CVPixelBuffer (we only need create-with-IOSurface here) ----

pub type CVPixelBufferRef = *mut c_void;
pub type IOSurfaceRef = *mut c_void;

extern "C" {
    pub fn CVPixelBufferCreateWithIOSurface(
        allocator: CFAllocatorRef,
        surface: IOSurfaceRef,
        pixel_buffer_attributes: CFDictionaryRef,
        pixel_buffer_out: *mut CVPixelBufferRef,
    ) -> i32;
}

// ---- VideoToolbox: VTCompressionSession ----

pub type VTCompressionSessionRef = *mut c_void;
pub type VTEncodeInfoFlags = u32;

/// Encode-completion callback signature. Invoked once per encoded frame.
pub type VTCompressionOutputCallback = unsafe extern "C" fn(
    output_callback_ref_con: *mut c_void,
    source_frame_ref_con: *mut c_void,
    status: OSStatus,
    info_flags: VTEncodeInfoFlags,
    sample_buffer: CMSampleBufferRef,
);

extern "C" {
    pub fn VTCompressionSessionCreate(
        allocator: CFAllocatorRef,
        width: i32,
        height: i32,
        codec_type: CMVideoCodecType,
        encoder_specification: CFDictionaryRef,
        source_image_buffer_attributes: CFDictionaryRef,
        compressed_data_allocator: CFAllocatorRef,
        output_callback: Option<VTCompressionOutputCallback>,
        output_callback_ref_con: *mut c_void,
        compression_session_out: *mut VTCompressionSessionRef,
    ) -> OSStatus;

    pub fn VTCompressionSessionInvalidate(session: VTCompressionSessionRef);

    pub fn VTCompressionSessionPrepareToEncodeFrames(session: VTCompressionSessionRef) -> OSStatus;

    pub fn VTCompressionSessionEncodeFrame(
        session: VTCompressionSessionRef,
        image_buffer: CVPixelBufferRef,
        presentation_time_stamp: CMTime,
        duration: CMTime,
        frame_properties: CFDictionaryRef,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut VTEncodeInfoFlags,
    ) -> OSStatus;

    pub fn VTCompressionSessionCompleteFrames(
        session: VTCompressionSessionRef,
        complete_until_presentation_time_stamp: CMTime,
    ) -> OSStatus;

    pub fn VTSessionSetProperty(
        session: VTCompressionSessionRef,
        property_key: CFStringRef,
        property_value: CFTypeRef,
    ) -> OSStatus;

    // Common compression property keys (declared as CFStringRef constants by the framework).
    pub static kVTCompressionPropertyKey_RealTime: CFStringRef;
    pub static kVTCompressionPropertyKey_AllowFrameReordering: CFStringRef;
    pub static kVTCompressionPropertyKey_AverageBitRate: CFStringRef;
    pub static kVTCompressionPropertyKey_ExpectedFrameRate: CFStringRef;
    pub static kVTCompressionPropertyKey_MaxKeyFrameInterval: CFStringRef;
    pub static kVTCompressionPropertyKey_ProfileLevel: CFStringRef;
    pub static kVTCompressionPropertyKey_H264EntropyMode: CFStringRef;
    pub static kVTCompressionPropertyKey_Quality: CFStringRef;

    pub static kVTProfileLevel_H264_Baseline_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_H264_Main_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_H264_High_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_HEVC_Main_AutoLevel: CFStringRef;
    pub static kVTProfileLevel_HEVC_Main10_AutoLevel: CFStringRef;

    pub static kVTH264EntropyMode_CABAC: CFStringRef;
    pub static kVTH264EntropyMode_CAVLC: CFStringRef;

    // ---- VTDecompressionSession (decoder side, added v0.3) ----
    pub fn VTDecompressionSessionCreate(
        allocator: CFAllocatorRef,
        video_format_description: CMFormatDescriptionRef,
        video_decoder_specification: CFDictionaryRef,
        destination_image_buffer_attributes: CFDictionaryRef,
        output_callback: *const VTDecompressionOutputCallbackRecord,
        decompression_session_out: *mut VTDecompressionSessionRef,
    ) -> OSStatus;

    pub fn VTDecompressionSessionInvalidate(session: VTDecompressionSessionRef);

    pub fn VTDecompressionSessionDecodeFrame(
        session: VTDecompressionSessionRef,
        sample_buffer: CMSampleBufferRef,
        decode_flags: u32,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut u32,
    ) -> OSStatus;

    pub fn VTDecompressionSessionWaitForAsynchronousFrames(
        session: VTDecompressionSessionRef,
    ) -> OSStatus;

    pub fn VTDecompressionSessionFinishDelayedFrames(
        session: VTDecompressionSessionRef,
    ) -> OSStatus;

    pub fn VTDecompressionSessionCanAcceptFormatDescription(
        session: VTDecompressionSessionRef,
        new_format_desc: CMFormatDescriptionRef,
    ) -> bool;

    pub static kVTDecompressionPropertyKey_RealTime: CFStringRef;
    pub static kVTDecompressionPropertyKey_MaximumOutputBufferDepth: CFStringRef;

    // ---- VTPixelTransferSession (v0.6) ----
    pub fn VTPixelTransferSessionCreate(
        allocator: CFAllocatorRef,
        pixel_transfer_session_out: *mut VTPixelTransferSessionRef,
    ) -> OSStatus;
    pub fn VTPixelTransferSessionInvalidate(session: VTPixelTransferSessionRef);
    pub fn VTPixelTransferSessionTransferImage(
        session: VTPixelTransferSessionRef,
        source_buffer: CVPixelBufferRef,
        destination_buffer: CVPixelBufferRef,
    ) -> OSStatus;

    // ---- VTPixelRotationSession (v0.6) ----
    pub fn VTPixelRotationSessionCreate(
        allocator: CFAllocatorRef,
        pixel_rotation_session_out: *mut VTPixelRotationSessionRef,
    ) -> OSStatus;
    pub fn VTPixelRotationSessionInvalidate(session: VTPixelRotationSessionRef);
    pub fn VTPixelRotationSessionRotateImage(
        session: VTPixelRotationSessionRef,
        source_buffer: CVPixelBufferRef,
        destination_buffer: CVPixelBufferRef,
    ) -> OSStatus;

    pub static kVTPixelRotationPropertyKey_Rotation: CFStringRef;
    pub static kVTPixelRotationPropertyKey_FlipHorizontalOrientation: CFStringRef;
    pub static kVTPixelRotationPropertyKey_FlipVerticalOrientation: CFStringRef;

    pub static kVTRotation_0: CFStringRef;
    pub static kVTRotation_CW90: CFStringRef;
    pub static kVTRotation_180: CFStringRef;
    pub static kVTRotation_CCW90: CFStringRef;
}

pub type VTPixelTransferSessionRef = *mut c_void;
pub type VTPixelRotationSessionRef = *mut c_void;

pub type VTDecompressionSessionRef = *mut c_void;
pub type VTDecompressionOutputCallback = unsafe extern "C" fn(
    decompression_output_ref_con: *mut c_void,
    source_frame_ref_con: *mut c_void,
    status: OSStatus,
    info_flags: u32,
    image_buffer: *mut c_void,
    presentation_time_stamp: CMTime,
    presentation_duration: CMTime,
);

#[repr(C)]
pub struct VTDecompressionOutputCallbackRecord {
    pub decompression_output_callback: VTDecompressionOutputCallback,
    pub decompression_output_ref_con: *mut c_void,
}

// Suppress unused variant warnings on c_uint placeholder types.
const _: () = {
    let _ = core::mem::size_of::<c_uint>();
};
