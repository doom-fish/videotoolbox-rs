//! `VTMotionEstimationSession` — between-frame motion-vector
//! estimation backed by the Apple Neural Engine (macOS 26+).
//!
//! Most of the surface is exposed through pure-C FFI; the only piece
//! that lives in the Swift bridge is the async `motion(of:comparedTo:)`
//! call, which Apple flags `CF_REFINED_FOR_SWIFT` and which uses
//! Swift concurrency under the hood.

use core::ffi::c_void;
use core::ptr;

use apple_cf::cv::CVPixelBuffer;

use crate::error::VTError;
use crate::ffi;

extern "C" {
    fn vtb_motion_session_create(width: u32, height: u32, out: *mut *mut c_void) -> i32;
    fn vtb_motion_session_release(session: *mut c_void);
    fn vtb_motion_session_estimate(
        session: *mut c_void,
        reference_image: *mut c_void,
        current_image: *mut c_void,
        out: *mut *mut c_void,
    ) -> i32;
}

/// `VTMotionEstimationSessionRef`.
pub struct MotionEstimationSession {
    inner: *mut c_void,
}

unsafe impl Send for MotionEstimationSession {}
unsafe impl Sync for MotionEstimationSession {}

impl Drop for MotionEstimationSession {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                ffi::VTMotionEstimationSessionInvalidate(self.inner);
                vtb_motion_session_release(self.inner);
            }
            self.inner = ptr::null_mut();
        }
    }
}

impl MotionEstimationSession {
    /// Create a motion-estimation session that accepts pixel buffers
    /// of the given size (must match for both `reference` and `current`).
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`] on failure or when
    /// running on macOS < 26.
    pub fn new(width: u32, height: u32) -> Result<Self, VTError> {
        let mut p: *mut c_void = ptr::null_mut();
        let s = unsafe { vtb_motion_session_create(width, height, &mut p) };
        if s != 0 || p.is_null() {
            return Err(VTError::SessionCreateFailed(s));
        }
        Ok(Self { inner: p })
    }

    /// Copy the source pixel-buffer attributes Apple recommends for
    /// feeding this session.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn source_pixel_buffer_attributes(&self) -> Result<*const c_void, VTError> {
        let mut attrs: ffi::CFDictionaryRef = ptr::null();
        let s = unsafe {
            ffi::VTMotionEstimationSessionCopySourcePixelBufferAttributes(self.inner, &mut attrs)
        };
        if s != 0 {
            return Err(VTError::EncodeFailed(s));
        }
        Ok(attrs)
    }

    /// Force-complete any outstanding estimations.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn complete_frames(&self) -> Result<(), VTError> {
        let s = unsafe { ffi::VTMotionEstimationSessionCompleteFrames(self.inner) };
        if s == 0 {
            Ok(())
        } else {
            Err(VTError::EncodeFailed(s))
        }
    }

    /// Estimate motion vectors from `reference` to `current`.
    /// Returns the resulting `CVPixelBuffer` of motion vectors.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn estimate(
        &self,
        reference: &CVPixelBuffer,
        current: &CVPixelBuffer,
    ) -> Result<CVPixelBuffer, VTError> {
        let mut out: *mut c_void = ptr::null_mut();
        let s = unsafe {
            vtb_motion_session_estimate(
                self.inner,
                reference.as_ptr().cast::<c_void>(),
                current.as_ptr().cast::<c_void>(),
                &mut out,
            )
        };
        if s != 0 || out.is_null() {
            return Err(VTError::EncodeFailed(s));
        }
        CVPixelBuffer::from_raw(out.cast()).ok_or(VTError::EncodeFailed(0))
    }

    /// Raw `VTMotionEstimationSessionRef`.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner
    }
}
