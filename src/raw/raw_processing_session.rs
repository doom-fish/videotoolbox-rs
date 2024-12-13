use std::ffi::c_void;

use core_foundation::{base::*, declare_TCFType, impl_CFTypeDescription, impl_TCFType};

#[repr(C)]
pub struct __VTRawProcessingSessionRef(c_void);

pub type VTRawProcessingSessionRef = *mut __VTRawProcessingSessionRef;

declare_TCFType! {VTRawProcessingSession, VTRawProcessingSessionRef}
impl_TCFType!(
    VTRawProcessingSession,
    VTRawProcessingSessionRef,
    VTRawProcessingSessionGetTypeID
);
impl_CFTypeDescription!(VTRawProcessingSession);

extern "C" {
    pub fn VTRawProcessingSessionGetTypeID() -> CFTypeID;
}
