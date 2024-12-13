use core_foundation::{base::*, declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use std::ffi::c_void;
#[repr(C)]
pub struct __VTDecompressionSessionRef(c_void);

pub type VTDecompressionSessionRef = *mut __VTDecompressionSessionRef;

declare_TCFType! {VTDecompressionSession, VTDecompressionSessionRef}
impl_TCFType!(
    VTDecompressionSession,
    VTDecompressionSessionRef,
    VTDecompressionSessionGetTypeID
);
impl_CFTypeDescription!(VTDecompressionSession);

extern "C" {
    pub fn VTDecompressionSessionGetTypeID() -> CFTypeID;
}
