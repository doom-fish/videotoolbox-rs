use std::ffi::c_void;

use core_foundation::{base::*, declare_TCFType, impl_CFTypeDescription, impl_TCFType};

#[repr(C)]
pub struct __VTPixelRotationSessionRef(c_void);

pub type VTPixelRotationSessionRef = *mut __VTPixelRotationSessionRef;

declare_TCFType! {VTPixelRotationSession, VTPixelRotationSessionRef}
impl_TCFType!(
    VTPixelRotationSession,
    VTPixelRotationSessionRef,
    VTPixelRotationSessionGetTypeID
);
impl_CFTypeDescription!(VTPixelRotationSession);

extern "C" {
    pub fn VTPixelRotationSessionGetTypeID() -> CFTypeID;
}
