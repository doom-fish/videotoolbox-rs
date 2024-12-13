use std::ffi::c_void;

use core_foundation::{base::*, declare_TCFType, impl_CFTypeDescription, impl_TCFType};

#[repr(C)]
pub struct __VTFrameSiloRef(c_void);

pub type VTFrameSiloRef = *mut __VTFrameSiloRef;

declare_TCFType! {VTFrameSilo, VTFrameSiloRef}
impl_TCFType!(VTFrameSilo, VTFrameSiloRef, VTFrameSiloGetTypeID);
impl_CFTypeDescription!(VTFrameSilo);

extern "C" {
    pub fn VTFrameSiloGetTypeID() -> CFTypeID;
}
