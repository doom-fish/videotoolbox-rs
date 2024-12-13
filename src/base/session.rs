use std::ffi::c_void;

use core_foundation::{base::*, declare_TCFType, impl_CFTypeDescription, impl_TCFType};

#[repr(C)]
pub struct __VTSessionRef(c_void);

pub type VTSessionRef = *mut __VTSessionRef;

declare_TCFType! {VTSession, VTSessionRef}
impl_TCFType!(VTSession, VTSessionRef, VTSessionGetTypeID);
impl_CFTypeDescription!(VTSession);

extern "C" {
    pub fn VTSessionGetTypeID() -> CFTypeID;
}
