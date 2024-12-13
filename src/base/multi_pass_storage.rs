use std::ffi::c_void;

use core_foundation::{base::*, declare_TCFType, impl_CFTypeDescription, impl_TCFType};

#[repr(C)]
pub struct __VTMultiPassStorageRef(c_void);

pub type VTMultiPassStorageRef = *mut __VTMultiPassStorageRef;

declare_TCFType! {VTMultiPassStorage, VTMultiPassStorageRef}
impl_TCFType!(
    VTMultiPassStorage,
    VTMultiPassStorageRef,
    VTMultiPassStorageGetTypeID
);
impl_CFTypeDescription!(VTMultiPassStorage);

extern "C" {
    pub fn VTMultiPassStorageGetTypeID() -> CFTypeID;
}
