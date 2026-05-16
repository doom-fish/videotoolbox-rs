//! `VTUtilities` + `VTProfessionalVideoWorkflow` helpers.

use core::ffi::c_void;
use core::ptr;

use apple_cf::cv::CVPixelBuffer;

use crate::error::VTError;
use crate::ffi;
use crate::session::Codec;

/// Convert a `CVPixelBuffer` into a `CGImageRef`. The returned
/// pointer is a retained `CGImageRef`; caller must `CFRelease` it
/// (e.g. via `apple_cf::cg::CGImage::from_raw`).
///
/// # Errors
///
/// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
pub fn create_cg_image_from_pixel_buffer(
    pixel_buffer: &CVPixelBuffer,
) -> Result<*mut c_void, VTError> {
    let mut img: *mut c_void = ptr::null_mut();
    let s = unsafe {
        ffi::VTCreateCGImageFromCVPixelBuffer(
            pixel_buffer.as_ptr().cast::<c_void>(),
            ptr::null(),
            &mut img,
        )
    };
    if s != 0 || img.is_null() {
        return Err(VTError::EncodeFailed(s));
    }
    Ok(img)
}

/// Returns `true` when the current machine advertises hardware decode support
/// for `codec`.
#[must_use]
pub fn is_hardware_decode_supported(codec: Codec) -> bool {
    unsafe { ffi::VTIsHardwareDecodeSupported(codec.as_cm_codec_type()) != 0 }
}

/// Register Apple's professional-workflow video decoders (extra
/// support for high-bit-depth `ProRes`, etc.). Safe to call multiple
/// times.
pub fn register_professional_workflow_decoders() {
    unsafe { ffi::VTRegisterProfessionalVideoWorkflowVideoDecoders() };
}

/// Register Apple's professional-workflow video encoders.
pub fn register_professional_workflow_encoders() {
    unsafe { ffi::VTRegisterProfessionalVideoWorkflowVideoEncoders() };
}
