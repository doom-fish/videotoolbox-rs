//! `VTFrameSilo` + `VTMultiPassStorage` — multi-pass video encoding storage.
//!
//! `VTFrameSilo` collects `CMSampleBuffer`s produced by a multi-pass
//! compression session and lets the encoder re-iterate over them for
//! subsequent passes. `VTMultiPassStorage` holds the encoder's private
//! per-pass scratch data (associate it with a `CompressionSession` via
//! `kVTCompressionPropertyKey_MultiPassStorage`).

use core::ffi::c_void;
use core::ptr;

use apple_cf::cm::CMSampleBuffer;

use crate::error::VTError;
use crate::ffi;

/// `VTFrameSiloRef` — keyed-by-PTS storage of encoded sample buffers.
pub struct FrameSilo {
    inner: ffi::VTFrameSiloRef,
}

unsafe impl Send for FrameSilo {}
unsafe impl Sync for FrameSilo {}

impl Drop for FrameSilo {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { ffi::CFRelease(self.inner.cast()) };
            self.inner = ptr::null_mut();
        }
    }
}

impl FrameSilo {
    /// Create an in-memory frame silo.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`] if Apple refuses.
    pub fn new() -> Result<Self, VTError> {
        let mut p: ffi::VTFrameSiloRef = ptr::null_mut();
        let s = unsafe {
            ffi::VTFrameSiloCreate(
                ffi::kCFAllocatorDefault,
                ptr::null(),
                ffi::CMTimeRange::INVALID,
                ptr::null(),
                &mut p,
            )
        };
        if s != 0 || p.is_null() {
            return Err(VTError::SessionCreateFailed(s));
        }
        Ok(Self { inner: p })
    }

    /// Append a `CMSampleBuffer` to the silo. Sample buffers must
    /// be added in ascending decode-timestamp order within a pass.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn add_sample_buffer(&self, sample: &CMSampleBuffer) -> Result<(), VTError> {
        let s = unsafe {
            ffi::VTFrameSiloAddSampleBuffer(self.inner, sample.as_ptr().cast::<c_void>())
        };
        if s == 0 {
            Ok(())
        } else {
            Err(VTError::EncodeFailed(s))
        }
    }

    /// Return Apple's progress estimate for the current pass
    /// (`0.0..=1.0`).
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn progress_of_current_pass(&self) -> Result<f32, VTError> {
        let mut p: f32 = 0.0;
        let s = unsafe { ffi::VTFrameSiloGetProgressOfCurrentPass(self.inner, &mut p) };
        if s == 0 {
            Ok(p)
        } else {
            Err(VTError::EncodeFailed(s))
        }
    }

    /// Raw `VTFrameSiloRef`.
    #[must_use]
    pub const fn as_ptr(&self) -> ffi::VTFrameSiloRef {
        self.inner
    }
}

/// `VTMultiPassStorageRef` — opaque encoder scratch storage for
/// multi-pass encoding.
pub struct MultiPassStorage {
    inner: ffi::VTMultiPassStorageRef,
}

unsafe impl Send for MultiPassStorage {}
unsafe impl Sync for MultiPassStorage {}

impl Drop for MultiPassStorage {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                let _ = ffi::VTMultiPassStorageClose(self.inner);
                ffi::CFRelease(self.inner.cast());
            }
            self.inner = ptr::null_mut();
        }
    }
}

impl MultiPassStorage {
    /// Create a new multi-pass storage backed by a temp file.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`] on failure.
    pub fn new() -> Result<Self, VTError> {
        let mut p: ffi::VTMultiPassStorageRef = ptr::null_mut();
        let s = unsafe {
            ffi::VTMultiPassStorageCreate(
                ffi::kCFAllocatorDefault,
                ptr::null(),
                ffi::CMTimeRange::INVALID,
                ptr::null(),
                &mut p,
            )
        };
        if s != 0 || p.is_null() {
            return Err(VTError::SessionCreateFailed(s));
        }
        Ok(Self { inner: p })
    }

    /// Raw `VTMultiPassStorageRef` — pass to a
    /// `CompressionSession::set_property` with
    /// `kVTCompressionPropertyKey_MultiPassStorage`.
    #[must_use]
    pub const fn as_ptr(&self) -> ffi::VTMultiPassStorageRef {
        self.inner
    }
}
