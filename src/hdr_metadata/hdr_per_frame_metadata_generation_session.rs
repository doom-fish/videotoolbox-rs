use std::ffi::c_void;

use core_foundation::{base::*, declare_TCFType, impl_CFTypeDescription, impl_TCFType};

#[repr(C)]
pub struct __VTHDRPerFrameMetadataGenerationSessionRef(c_void);

pub type VTHDRPerFrameMetadataGenerationSessionRef =
    *mut __VTHDRPerFrameMetadataGenerationSessionRef;

declare_TCFType! {VTHDRPerFrameMetadataGenerationSession, VTHDRPerFrameMetadataGenerationSessionRef}
impl_TCFType!(
    VTHDRPerFrameMetadataGenerationSession,
    VTHDRPerFrameMetadataGenerationSessionRef,
    VTHDRPerFrameMetadataGenerationSessionGetTypeID
);
impl_CFTypeDescription!(VTHDRPerFrameMetadataGenerationSession);

extern "C" {
    pub fn VTHDRPerFrameMetadataGenerationSessionGetTypeID() -> CFTypeID;
}
