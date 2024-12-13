use core_foundation::{
    base::*,
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
};
use core_media_rs::cm_video_codec_type::CMVideoCodecType;
use std::ffi::c_void;
#[repr(C)]
pub struct __VTCompressionSessionRef(c_void);

pub type VTCompressionSessionRef = *mut __VTCompressionSessionRef;

declare_TCFType! {VTCompressionSession, VTCompressionSessionRef}
impl_TCFType!(
    VTCompressionSession,
    VTCompressionSessionRef,
    VTCompressionSessionGetTypeID
);
impl_CFTypeDescription!(VTCompressionSession);

extern "C" {
    pub fn VTCompressionSessionGetTypeID() -> CFTypeID;
    pub fn VTCompressionSessionCreate(
        allocator: CFAllocatorRef,
        width: u32,
        height: u32,
        codecType: u32,
        encoderSpecification: CFDictionaryRef,
        sourceImageBufferAttributes: CFDictionaryRef,
        compressedDataAllocator: CFAllocatorRef,
        // outputCallback: Option<VTCompressionOutputCallback>,
        refcon: *mut c_void,
        compressionSessionOut: *mut VTCompressionSessionRef,
    ) -> OSStatus;
}

impl VTCompressionSession {
    fn new(
        width: u32,
        height: u32,
        codec_type: CMVideoCodecType,
        encoder_specification: &CFDictionary,
        source_image_buffer_attributes: &CFDictionary,
        compressed_data_allocator: &CFAllocator,
        // output_callback: Option<VTCompressionOutputCallback>,
        refcon: *mut c_void,
    ) -> Self {
        unsafe {
            let mut session: VTCompressionSessionRef = std::ptr::null_mut();
            let status = VTCompressionSessionCreate(
                kCFAllocatorDefault,
                width,
                height,
                codec_type.into(),
                encoder_specification.as_concrete_TypeRef(),
                source_image_buffer_attributes.as_concrete_TypeRef(),
                compressed_data_allocator.as_concrete_TypeRef(),
                std::ptr::null_mut(),
                &mut session,
            );
            if status != 0 {
                panic!("Failed to create compression session");
            }
            VTCompressionSession(session)
        }
    }
}
