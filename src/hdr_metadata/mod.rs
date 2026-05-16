//! `VTHDRPerFrameMetadataGenerationSession` — generate Dolby Vision
//! per-frame HDR metadata (macOS 15+).

use core::ffi::c_void;
use core::ptr;

use apple_cf::cv::CVPixelBuffer;

use crate::error::VTError;
use crate::ffi;

/// `VTHDRPerFrameMetadataGenerationSessionRef`.
pub struct HdrMetadataSession {
    inner: ffi::VTHDRPerFrameMetadataGenerationSessionRef,
}

unsafe impl Send for HdrMetadataSession {}
unsafe impl Sync for HdrMetadataSession {}

impl Drop for HdrMetadataSession {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { ffi::CFRelease(self.inner.cast()) };
            self.inner = ptr::null_mut();
        }
    }
}

impl HdrMetadataSession {
    /// Create a new HDR-metadata generation session. `fps` is the
    /// source frame rate (used for temporal-coherence calculations).
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`] on failure.
    pub fn new(fps: f32) -> Result<Self, VTError> {
        let mut p: ffi::VTHDRPerFrameMetadataGenerationSessionRef = ptr::null_mut();
        let s = unsafe {
            ffi::VTHDRPerFrameMetadataGenerationSessionCreate(
                ffi::kCFAllocatorDefault,
                fps,
                ptr::null(),
                &mut p,
            )
        };
        if s != 0 || p.is_null() {
            return Err(VTError::SessionCreateFailed(s));
        }
        Ok(Self { inner: p })
    }

    /// Analyze `pixel_buffer` and attach generated HDR metadata to
    /// its `CVPixelBuffer` attachments and backing `IOSurface`.
    /// Set `scene_change = true` when the frame is a hard scene
    /// transition.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn attach_metadata(
        &self,
        pixel_buffer: &CVPixelBuffer,
        scene_change: bool,
    ) -> Result<(), VTError> {
        let s = unsafe {
            ffi::VTHDRPerFrameMetadataGenerationSessionAttachMetadata(
                self.inner,
                pixel_buffer.as_ptr().cast::<c_void>(),
                scene_change,
            )
        };
        if s == 0 {
            Ok(())
        } else {
            Err(VTError::EncodeFailed(s))
        }
    }

    /// Raw `VTHDRPerFrameMetadataGenerationSessionRef`.
    #[must_use]
    pub const fn as_ptr(&self) -> ffi::VTHDRPerFrameMetadataGenerationSessionRef {
        self.inner
    }
}
