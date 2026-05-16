#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]

//! `VTRAWProcessingSession` — `ProRes` RAW / `CinemaDNG` decoder with
//! per-frame parameter controls (macOS 15+).
//!
//! Session creation / parameter readback / parameter writeback /
//! complete-frames all go through the pure-C FFI. Only the async
//! `process(frame:)` call lives in the Swift bridge.

use core::ffi::c_void;
use core::ptr;

use apple_cf::cm::CMFormatDescription;
use apple_cf::cv::CVPixelBuffer;

use crate::error::VTError;
use crate::ffi;

extern "C" {
    fn vtb_raw_session_process_frame(
        session: *mut c_void,
        input_pixel_buffer: *mut c_void,
        out: *mut *mut c_void,
    ) -> i32;
}

/// `VTRAWProcessingSessionRef`.
pub struct RawProcessingSession {
    inner: ffi::VTRAWProcessingSessionRef,
}

unsafe impl Send for RawProcessingSession {}
unsafe impl Sync for RawProcessingSession {}

impl Drop for RawProcessingSession {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                ffi::VTRAWProcessingSessionInvalidate(self.inner);
                ffi::CFRelease(self.inner.cast());
            }
            self.inner = ptr::null_mut();
        }
    }
}

impl RawProcessingSession {
    /// Create a RAW processing session for the given video format.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`] on failure.
    pub fn new(format: &CMFormatDescription) -> Result<Self, VTError> {
        let mut p: ffi::VTRAWProcessingSessionRef = ptr::null_mut();
        let s = unsafe {
            ffi::VTRAWProcessingSessionCreate(
                ffi::kCFAllocatorDefault,
                format.as_ptr().cast::<c_void>(),
                ptr::null(),
                ptr::null(),
                &mut p,
            )
        };
        if s != 0 || p.is_null() {
            return Err(VTError::SessionCreateFailed(s));
        }
        Ok(Self { inner: p })
    }

    /// Force-complete any outstanding frames.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn complete_frames(&self) -> Result<(), VTError> {
        let s = unsafe { ffi::VTRAWProcessingSessionCompleteFrames(self.inner) };
        if s == 0 {
            Ok(())
        } else {
            Err(VTError::EncodeFailed(s))
        }
    }

    /// Copy the array of processing parameters this RAW codec
    /// exposes.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn parameters(&self) -> Result<Vec<RawProcessingParameter>, VTError> {
        let mut arr: ffi::CFArrayRef = ptr::null();
        let s =
            unsafe { ffi::VTRAWProcessingSessionCopyProcessingParameters(self.inner, &mut arr) };
        if s != 0 || arr.is_null() {
            return Err(VTError::EncodeFailed(s));
        }
        let count = unsafe { ffi::CFArrayGetCount(arr) };
        let mut out = Vec::with_capacity(count.max(0) as usize);
        for i in 0..count {
            let dict = unsafe { ffi::CFArrayGetValueAtIndex(arr, i) };
            if !dict.is_null() {
                unsafe { ffi::CFRetain(dict.cast()) };
                out.push(RawProcessingParameter {
                    dict: dict.cast_mut(),
                });
            }
        }
        unsafe { ffi::CFRelease(arr.cast()) };
        Ok(out)
    }

    /// Set processing parameters. The dictionary keys must match the
    /// `kVTRAWProcessingParameter_Key` strings returned by
    /// [`parameters`](Self::parameters); values must conform to the
    /// declared value type.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    ///
    /// # Safety
    ///
    /// `params` must be a valid `CFDictionaryRef`.
    pub unsafe fn set_parameters_raw(&self, params: ffi::CFDictionaryRef) -> Result<(), VTError> {
        let s = unsafe { ffi::VTRAWProcessingSessionSetProcessingParameters(self.inner, params) };
        if s == 0 {
            Ok(())
        } else {
            Err(VTError::EncodeFailed(s))
        }
    }

    /// Process a single RAW input frame, returning the processed
    /// `CVPixelBuffer`. Blocks on Swift's `process(frame:)` async
    /// call internally.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn process(&self, input: &CVPixelBuffer) -> Result<CVPixelBuffer, VTError> {
        let mut out: *mut c_void = ptr::null_mut();
        let s = unsafe {
            vtb_raw_session_process_frame(self.inner, input.as_ptr().cast::<c_void>(), &mut out)
        };
        if s != 0 || out.is_null() {
            return Err(VTError::EncodeFailed(s));
        }
        CVPixelBuffer::from_raw(out.cast()).ok_or(VTError::EncodeFailed(0))
    }

    /// Raw `VTRAWProcessingSessionRef`.
    #[must_use]
    pub const fn as_ptr(&self) -> ffi::VTRAWProcessingSessionRef {
        self.inner
    }
}

/// A single RAW-processing parameter descriptor.
///
/// Wraps a `CFDictionary` returned by `VTRAWProcessingSession::parameters`.
pub struct RawProcessingParameter {
    dict: *mut c_void,
}

unsafe impl Send for RawProcessingParameter {}
unsafe impl Sync for RawProcessingParameter {}

impl Drop for RawProcessingParameter {
    fn drop(&mut self) {
        if !self.dict.is_null() {
            unsafe { ffi::CFRelease(self.dict.cast()) };
            self.dict = ptr::null_mut();
        }
    }
}

impl RawProcessingParameter {
    /// Raw `CFDictionaryRef`.
    #[must_use]
    pub const fn as_ptr(&self) -> ffi::CFDictionaryRef {
        self.dict
    }

    /// Stable identifier used as the key when writing back via
    /// [`RawProcessingSession::set_parameters_raw`].
    #[must_use]
    pub fn key(&self) -> Option<String> {
        unsafe { self.cf_string(ffi::kVTRAWProcessingParameter_Key) }
    }

    /// Localised, human-readable name.
    #[must_use]
    pub fn name(&self) -> Option<String> {
        unsafe { self.cf_string(ffi::kVTRAWProcessingParameter_Name) }
    }

    /// Long-form description.
    #[must_use]
    pub fn description(&self) -> Option<String> {
        unsafe { self.cf_string(ffi::kVTRAWProcessingParameter_Description) }
    }

    /// `kVTRAWProcessingParameterValueType_*` discriminator.
    #[must_use]
    pub fn value_type(&self) -> Option<String> {
        unsafe { self.cf_string(ffi::kVTRAWProcessingParameter_ValueType) }
    }

    /// Current value (numeric).
    #[must_use]
    pub fn current_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_CurrentValue) }
    }

    /// Minimum value (numeric).
    #[must_use]
    pub fn minimum_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_MinimumValue) }
    }

    /// Maximum value (numeric).
    #[must_use]
    pub fn maximum_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_MaximumValue) }
    }

    /// Initial/default value.
    #[must_use]
    pub fn initial_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_InitialValue) }
    }

    /// Camera-captured value.
    #[must_use]
    pub fn camera_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_CameraValue) }
    }

    /// Neutral (no-op) value.
    #[must_use]
    pub fn neutral_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_NeutralValue) }
    }

    unsafe fn cf_string(&self, key: ffi::CFStringRef) -> Option<String> {
        let v = unsafe { ffi::CFDictionaryGetValue(self.dict, key.cast()) };
        if v.is_null() {
            return None;
        }
        let len = unsafe { ffi::CFStringGetLength(v) };
        if len < 0 {
            return None;
        }
        let cap = (len as usize).saturating_mul(4) + 1;
        let mut buf = vec![0u8; cap];
        let ok = unsafe {
            ffi::CFStringGetCString(
                v,
                buf.as_mut_ptr().cast(),
                cap as isize,
                ffi::kCFStringEncodingUTF8,
            )
        };
        if !ok {
            return None;
        }
        let nul = buf.iter().position(|&b| b == 0).unwrap_or(cap);
        Some(String::from_utf8_lossy(&buf[..nul]).into_owned())
    }

    unsafe fn cf_f64(&self, key: ffi::CFStringRef) -> Option<f64> {
        let v = unsafe { ffi::CFDictionaryGetValue(self.dict, key.cast()) };
        if v.is_null() {
            return None;
        }
        let mut out: f64 = 0.0;
        let ok = unsafe {
            ffi::CFNumberGetValue(v, ffi::kCFNumberFloat64Type, (&raw mut out).cast())
        };
        if ok {
            Some(out)
        } else {
            None
        }
    }
}
