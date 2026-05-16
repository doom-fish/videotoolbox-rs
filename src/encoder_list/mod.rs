//! [`available_video_encoders`] — enumerate Apple's installed video
//! encoders. Wraps `VTCopyVideoEncoderList`.

use core::ffi::{c_char, c_void};
use core::ptr;

use crate::ffi;

/// One entry from `VTCopyVideoEncoderList`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoEncoder {
    /// `CMVideoCodecType` four-character code (e.g. `0x61766331` for `'avc1'`).
    pub codec_type: u32,
    /// Reverse-DNS unique encoder id (e.g.
    /// `"com.apple.videotoolbox.videoencoder.h264"`).
    pub encoder_id: String,
    /// Codec name (e.g. `"H.264"`).
    pub codec_name: String,
    /// Encoder name (e.g. `"Apple H.264"`).
    pub encoder_name: String,
    /// Display name (codec name if there's only one encoder for that
    /// format, otherwise the encoder name).
    pub display_name: String,
}

/// Enumerate all video encoders installed on this system.
///
/// # Errors
///
/// Returns the raw `OSStatus` from `VTCopyVideoEncoderList` on
/// failure.
pub fn available_video_encoders() -> Result<Vec<VideoEncoder>, i32> {
    let mut arr: ffi::CFArrayRef = ptr::null();
    let status = unsafe { ffi::VTCopyVideoEncoderList(ptr::null(), &mut arr) };
    if status != 0 {
        return Err(status);
    }
    if arr.is_null() {
        return Ok(Vec::new());
    }
    let n = unsafe { ffi::CFArrayGetCount(arr) };
    let mut v = Vec::with_capacity(usize::try_from(n).unwrap_or(0));
    for i in 0..n {
        let dict = unsafe { ffi::CFArrayGetValueAtIndex(arr, i) }.cast::<c_void>();
        if dict.is_null() {
            continue;
        }
        let codec_type = unsafe {
            let n = ffi::CFDictionaryGetValue(dict, ffi::kVTVideoEncoderList_CodecType.cast());
            cf_number_u32(n).unwrap_or(0)
        };
        let encoder_id = unsafe {
            let s = ffi::CFDictionaryGetValue(dict, ffi::kVTVideoEncoderList_EncoderID.cast());
            cf_string_to_rust(s.cast())
        };
        let codec_name = unsafe {
            let s = ffi::CFDictionaryGetValue(dict, ffi::kVTVideoEncoderList_CodecName.cast());
            cf_string_to_rust(s.cast())
        };
        let encoder_name = unsafe {
            let s = ffi::CFDictionaryGetValue(dict, ffi::kVTVideoEncoderList_EncoderName.cast());
            cf_string_to_rust(s.cast())
        };
        let display_name = unsafe {
            let s = ffi::CFDictionaryGetValue(dict, ffi::kVTVideoEncoderList_DisplayName.cast());
            cf_string_to_rust(s.cast())
        };
        v.push(VideoEncoder {
            codec_type,
            encoder_id,
            codec_name,
            encoder_name,
            display_name,
        });
    }
    unsafe { ffi::CFRelease(arr) };
    Ok(v)
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
unsafe fn cf_string_to_rust(s: ffi::CFStringRef) -> String {
    if s.is_null() {
        return String::new();
    }
    let len = unsafe { ffi::CFStringGetLength(s) };
    let buf_len = usize::try_from(len * 4 + 1).unwrap_or(0);
    let mut buf = vec![0u8; buf_len];
    let ok = unsafe {
        ffi::CFStringGetCString(
            s,
            buf.as_mut_ptr().cast::<c_char>(),
            buf_len as isize,
            0x0800_0100,
        )
    };
    if !ok {
        return String::new();
    }
    if let Some(end) = buf.iter().position(|&b| b == 0) {
        buf.truncate(end);
    }
    String::from_utf8_lossy(&buf).into_owned()
}

#[allow(clippy::cast_sign_loss)]
unsafe fn cf_number_u32(n: *const c_void) -> Option<u32> {
    if n.is_null() {
        return None;
    }
    let mut v: i32 = 0;
    let ok = unsafe { ffi::CFNumberGetValue(n.cast(), 3, (&raw mut v).cast::<c_void>()) };
    if ok {
        Some(v as u32)
    } else {
        None
    }
}
