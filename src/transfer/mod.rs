//! [`PixelTransferSession`] and [`PixelRotationSession`] — Apple's
//! zero-copy pixel-format conversion / scaling / rotation engines.
//!
//! Both are typically used as pre/post-encode stages in a video
//! pipeline:
//!
//! - `PixelTransferSession` copies + converts (and optionally scales)
//!   one `CVPixelBuffer` into another. Pixel format, color space, and
//!   dimensions of the destination buffer determine what conversion
//!   is performed.
//! - `PixelRotationSession` rotates one `CVPixelBuffer` into another
//!   in 90° increments (with optional horizontal/vertical flips).
//!   For 90°/270°, the destination's width and height must be swapped.

use core::ffi::c_void;
use core::ptr;

use apple_cf::cv::CVPixelBuffer;

use crate::error::VTError;
use crate::ffi;

/// `kVTRotation_*` enum values, mapped to a Rust enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    None,
    Clockwise90,
    Half180,
    CounterClockwise90,
}

impl Rotation {
    fn as_cf_string(self) -> ffi::CFStringRef {
        unsafe {
            match self {
                Self::None => ffi::kVTRotation_0,
                Self::Clockwise90 => ffi::kVTRotation_CW90,
                Self::Half180 => ffi::kVTRotation_180,
                Self::CounterClockwise90 => ffi::kVTRotation_CCW90,
            }
        }
    }
}

/// Apple's `VTPixelTransferSession` — color/format/scale conversion
/// for `CVPixelBuffer`s.
pub struct PixelTransferSession {
    session: ffi::VTPixelTransferSessionRef,
}

unsafe impl Send for PixelTransferSession {}
unsafe impl Sync for PixelTransferSession {}

impl Drop for PixelTransferSession {
    fn drop(&mut self) {
        if !self.session.is_null() {
            unsafe {
                ffi::VTPixelTransferSessionInvalidate(self.session);
                ffi::CFRelease(self.session.cast());
            }
            self.session = ptr::null_mut();
        }
    }
}

impl PixelTransferSession {
    /// Create a new pixel transfer session.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`] if Apple refuses.
    pub fn new() -> Result<Self, VTError> {
        let mut session: ffi::VTPixelTransferSessionRef = ptr::null_mut();
        let status =
            unsafe { ffi::VTPixelTransferSessionCreate(ffi::kCFAllocatorDefault, &mut session) };
        if status != 0 || session.is_null() {
            return Err(VTError::SessionCreateFailed(status));
        }
        Ok(Self { session })
    }

    /// Copy + convert pixels from `src` into `dst`. The destination's
    /// pixel format, color space, and dimensions determine the
    /// conversion / scaling that gets applied.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on a non-zero `OSStatus`.
    pub fn transfer(&self, src: &CVPixelBuffer, dst: &CVPixelBuffer) -> Result<(), VTError> {
        let status = unsafe {
            ffi::VTPixelTransferSessionTransferImage(
                self.session,
                src.as_ptr().cast::<c_void>(),
                dst.as_ptr().cast::<c_void>(),
            )
        };
        if status == 0 {
            Ok(())
        } else {
            Err(VTError::EncodeFailed(status))
        }
    }

    /// Set an arbitrary `VTSession` property on this transfer session.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SetPropertyFailed`] on rejection.
    ///
    /// # Safety
    ///
    /// `key` / `value` must be valid CoreFoundation references
    /// matching Apple's `VTPixelTransferProperties.h` contract.
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

/// Apple's `VTPixelRotationSession` — 90° rotation + flip engine for
/// `CVPixelBuffer`s.
pub struct PixelRotationSession {
    session: ffi::VTPixelRotationSessionRef,
}

unsafe impl Send for PixelRotationSession {}
unsafe impl Sync for PixelRotationSession {}

impl Drop for PixelRotationSession {
    fn drop(&mut self) {
        if !self.session.is_null() {
            unsafe {
                ffi::VTPixelRotationSessionInvalidate(self.session);
                ffi::CFRelease(self.session.cast());
            }
            self.session = ptr::null_mut();
        }
    }
}

impl PixelRotationSession {
    /// Create a new pixel rotation session.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`] if Apple refuses.
    pub fn new() -> Result<Self, VTError> {
        let mut session: ffi::VTPixelRotationSessionRef = ptr::null_mut();
        let status =
            unsafe { ffi::VTPixelRotationSessionCreate(ffi::kCFAllocatorDefault, &mut session) };
        if status != 0 || session.is_null() {
            return Err(VTError::SessionCreateFailed(status));
        }
        Ok(Self { session })
    }

    /// Configure the rotation applied by [`Self::rotate`]. Wraps
    /// `kVTPixelRotationPropertyKey_Rotation`.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SetPropertyFailed`] on rejection.
    pub fn set_rotation(&self, rotation: Rotation) -> Result<(), VTError> {
        let v = rotation.as_cf_string();
        let status = unsafe {
            ffi::VTSessionSetProperty(self.session, ffi::kVTPixelRotationPropertyKey_Rotation, v.cast())
        };
        if status != 0 {
            return Err(VTError::SetPropertyFailed {
                key: "Rotation".to_string(),
                status,
            });
        }
        Ok(())
    }

    /// Toggle horizontal flip. Wraps
    /// `kVTPixelRotationPropertyKey_FlipHorizontalOrientation`.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SetPropertyFailed`] on rejection.
    pub fn set_flip_horizontal(&self, flip: bool) -> Result<(), VTError> {
        let cf = unsafe {
            if flip {
                ffi::kCFBooleanTrue
            } else {
                ffi::kCFBooleanFalse
            }
        };
        let status = unsafe {
            ffi::VTSessionSetProperty(
                self.session,
                ffi::kVTPixelRotationPropertyKey_FlipHorizontalOrientation,
                cf,
            )
        };
        if status != 0 {
            return Err(VTError::SetPropertyFailed {
                key: "FlipHorizontalOrientation".to_string(),
                status,
            });
        }
        Ok(())
    }

    /// Toggle vertical flip. Wraps
    /// `kVTPixelRotationPropertyKey_FlipVerticalOrientation`.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SetPropertyFailed`] on rejection.
    pub fn set_flip_vertical(&self, flip: bool) -> Result<(), VTError> {
        let cf = unsafe {
            if flip {
                ffi::kCFBooleanTrue
            } else {
                ffi::kCFBooleanFalse
            }
        };
        let status = unsafe {
            ffi::VTSessionSetProperty(
                self.session,
                ffi::kVTPixelRotationPropertyKey_FlipVerticalOrientation,
                cf,
            )
        };
        if status != 0 {
            return Err(VTError::SetPropertyFailed {
                key: "FlipVerticalOrientation".to_string(),
                status,
            });
        }
        Ok(())
    }

    /// Rotate (and optionally flip) `src` into `dst`. For 90°/270°
    /// rotations the destination buffer's width and height must be
    /// the inverse of the source's.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on a non-zero `OSStatus`.
    pub fn rotate(&self, src: &CVPixelBuffer, dst: &CVPixelBuffer) -> Result<(), VTError> {
        let status = unsafe {
            ffi::VTPixelRotationSessionRotateImage(
                self.session,
                src.as_ptr().cast::<c_void>(),
                dst.as_ptr().cast::<c_void>(),
            )
        };
        if status == 0 {
            Ok(())
        } else {
            Err(VTError::EncodeFailed(status))
        }
    }
}
