use std::ffi::c_void;

use core_foundation::{base::*, declare_TCFType, impl_CFTypeDescription, impl_TCFType};

#[repr(C)]
pub struct __VTPixelTransferSessionRef(c_void);

pub type VTPixelTransferSessionRef = *mut __VTPixelTransferSessionRef;

declare_TCFType! {VTPixelTransferSession, VTPixelTransferSessionRef}
impl_TCFType!(
    VTPixelTransferSession,
    VTPixelTransferSessionRef,
    VTPixelTransferSessionGetTypeID
);
impl_CFTypeDescription!(VTPixelTransferSession);

extern "C" {
    pub fn VTPixelTransferSessionGetTypeID() -> CFTypeID;
}
