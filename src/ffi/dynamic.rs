#![allow(non_snake_case)]

use core::ffi::{c_char, c_void};
use core::ptr::NonNull;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::error::VTError;

extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

const RTLD_DEFAULT: *mut c_void = -2_isize as *mut c_void;
const UNRESOLVED: *mut c_void = NonNull::<c_void>::dangling().as_ptr();

pub struct Symbol {
    name: &'static str,
    minimum: &'static str,
    address: AtomicPtr<c_void>,
}

impl Symbol {
    pub const fn new(name: &'static str, minimum: &'static str) -> Self {
        Self {
            name,
            minimum,
            address: AtomicPtr::new(UNRESOLVED),
        }
    }

    pub fn resolve(&self) -> Result<NonNull<c_void>, VTError> {
        let mut address = self.address.load(Ordering::Acquire);
        if address == UNRESOLVED {
            address = unsafe { dlsym(RTLD_DEFAULT, self.name.as_ptr().cast()) };
            self.address.store(address, Ordering::Release);
        }
        NonNull::new(address).ok_or_else(|| VTError::Unsupported {
            api: self.name.trim_end_matches('\0'),
            minimum: self.minimum,
        })
    }
}

macro_rules! dynamic_functions {
    ($($(#[$attr:meta])* $minimum:literal $name:ident: fn($($arg:ty),* $(,)?) $(-> $ret:ty)?;)+) => {
        $(
            $(#[$attr])*
            pub fn $name() -> Result<unsafe extern "C" fn($($arg),*) $(-> $ret)?, VTError> {
                static SYMBOL: Symbol = Symbol::new(concat!(stringify!($name), "\0"), $minimum);
                SYMBOL.resolve().map(|address| unsafe {
                    core::mem::transmute::<*mut c_void, unsafe extern "C" fn($($arg),*) $(-> $ret)?>(
                        address.as_ptr(),
                    )
                })
            }
        )+
    };
}

macro_rules! dynamic_constants {
    ($($(#[$attr:meta])* $minimum:literal $name:ident: $ty:ty;)+) => {
        $(
            $(#[$attr])*
            pub fn $name() -> Result<$ty, VTError> {
                static SYMBOL: Symbol = Symbol::new(concat!(stringify!($name), "\0"), $minimum);
                SYMBOL
                    .resolve()
                    .map(|address| unsafe { address.cast::<$ty>().as_ptr().read() })
            }
        )+
    };
}

dynamic_functions! {
    "14.0" CMTaggedBufferGroupGetTypeID: fn() -> usize;
    "14.0" CMTaggedBufferGroupGetCount: fn(super::CMTaggedBufferGroupRef) -> super::CMItemCount;
    "14.0" CMTaggedBufferGroupGetCVPixelBufferAtIndex:
        fn(super::CMTaggedBufferGroupRef, isize) -> super::CVPixelBufferRef;
    "14.0" CMTaggedBufferGroupGetCMSampleBufferAtIndex:
        fn(super::CMTaggedBufferGroupRef, isize) -> super::CMSampleBufferRef;
    #[cfg(feature = "compression")]
    "14.0" VTCompressionSessionEncodeMultiImageFrame: fn(
        super::VTCompressionSessionRef,
        super::CMTaggedBufferGroupRef,
        super::CMTime,
        super::CMTime,
        super::CFDictionaryRef,
        *mut c_void,
        *mut super::VTEncodeInfoFlags,
    ) -> super::OSStatus;
    #[cfg(feature = "compression")]
    "14.0" VTIsStereoMVHEVCEncodeSupported: fn() -> super::Boolean;
    "14.0" VTIsStereoMVHEVCDecodeSupported: fn() -> super::Boolean;
    "14.0" VTDecompressionSessionSetMultiImageCallback: fn(
        super::VTDecompressionSessionRef,
        super::VTDecompressionOutputMultiImageCallback,
        *mut c_void,
    ) -> super::OSStatus;
    "15.0" VTDecompressionSessionDecodeFrameWithOptions: fn(
        super::VTDecompressionSessionRef,
        super::CMSampleBufferRef,
        super::VTDecodeFrameFlags,
        super::CFDictionaryRef,
        *mut c_void,
        *mut super::VTDecodeInfoFlags,
    ) -> super::OSStatus;
    "15.0" VTCopyVideoDecoderExtensionProperties:
        fn(super::CMFormatDescriptionRef, *mut super::CFDictionaryRef) -> super::OSStatus;
    "15.0" VTCopyRAWProcessorExtensionProperties:
        fn(super::CMFormatDescriptionRef, *mut super::CFDictionaryRef) -> super::OSStatus;
    "15.0" VTHDRPerFrameMetadataGenerationSessionGetTypeID: fn() -> usize;
    "15.0" VTHDRPerFrameMetadataGenerationSessionCreate: fn(
        super::CFAllocatorRef,
        f32,
        super::CFDictionaryRef,
        *mut super::VTHDRPerFrameMetadataGenerationSessionRef,
    ) -> super::OSStatus;
    "15.0" VTHDRPerFrameMetadataGenerationSessionAttachMetadata: fn(
        super::VTHDRPerFrameMetadataGenerationSessionRef,
        super::CVPixelBufferRef,
        super::Boolean,
    ) -> super::OSStatus;
    #[cfg(feature = "frame_processor")]
    "15.0" VTRAWProcessingSessionGetTypeID: fn() -> usize;
    #[cfg(feature = "frame_processor")]
    "15.0" VTRAWProcessingSessionCreate: fn(
        super::CFAllocatorRef,
        *const c_void,
        super::CFDictionaryRef,
        super::CFDictionaryRef,
        *mut super::VTRAWProcessingSessionRef,
    ) -> super::OSStatus;
    #[cfg(feature = "frame_processor")]
    "15.0" VTRAWProcessingSessionInvalidate: fn(super::VTRAWProcessingSessionRef);
    #[cfg(feature = "frame_processor")]
    "15.0" VTRAWProcessingSessionCompleteFrames:
        fn(super::VTRAWProcessingSessionRef) -> super::OSStatus;
    #[cfg(feature = "frame_processor")]
    "15.0" VTRAWProcessingSessionCopyProcessingParameters:
        fn(super::VTRAWProcessingSessionRef, *mut super::CFArrayRef) -> super::OSStatus;
    #[cfg(feature = "frame_processor")]
    "15.0" VTRAWProcessingSessionSetProcessingParameters:
        fn(super::VTRAWProcessingSessionRef, super::CFDictionaryRef) -> super::OSStatus;
    #[cfg(feature = "frame_processor")]
    "26.0" VTMotionEstimationSessionGetTypeID: fn() -> usize;
    #[cfg(feature = "frame_processor")]
    "26.0" VTMotionEstimationSessionCreate: fn(
        super::CFAllocatorRef,
        super::CFDictionaryRef,
        u32,
        u32,
        *mut super::VTMotionEstimationSessionRef,
    ) -> super::OSStatus;
    #[cfg(feature = "frame_processor")]
    "26.0" VTMotionEstimationSessionInvalidate: fn(super::VTMotionEstimationSessionRef);
    #[cfg(feature = "frame_processor")]
    "26.0" VTMotionEstimationSessionCopySourcePixelBufferAttributes:
        fn(super::VTMotionEstimationSessionRef, *mut super::CFDictionaryRef) -> super::OSStatus;
    #[cfg(feature = "frame_processor")]
    "26.0" VTMotionEstimationSessionCompleteFrames:
        fn(super::VTMotionEstimationSessionRef) -> super::OSStatus;
}

dynamic_constants! {
    "15.0" kVTHDRPerFrameMetadataGenerationHDRFormatType_DolbyVision:
        super::VTHDRPerFrameMetadataGenerationHDRFormatType;
    "15.0" kVTHDRPerFrameMetadataGenerationOptionsKey_HDRFormats: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_Key: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_Name: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_Description: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_ValueType: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_CurrentValue: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_MinimumValue: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_MaximumValue: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_InitialValue: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_CameraValue: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingParameter_NeutralValue: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingPropertyKey_MetalDeviceRegistryID: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "15.0" kVTRAWProcessingPropertyKey_OutputColorAttachments: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "26.0" kVTRAWProcessingPropertyKey_MetadataForSidecarFile: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "26.0" kVTMotionEstimationSessionCreationOption_MotionVectorSize: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "26.0" kVTMotionEstimationSessionCreationOption_UseMultiPassSearch: super::CFStringRef;
    #[cfg(feature = "frame_processor")]
    "26.0" kVTMotionEstimationSessionCreationOption_Label: super::CFStringRef;
}

#[cfg(test)]
mod tests {
    use super::Symbol;
    use crate::error::VTError;

    #[test]
    fn missing_symbol_reports_the_api_and_minimum() {
        static SYMBOL: Symbol = Symbol::new("VTNoSuchFunctionForTesting\0", "99.0");

        assert_eq!(
            SYMBOL.resolve(),
            Err(VTError::Unsupported {
                api: "VTNoSuchFunctionForTesting",
                minimum: "99.0",
            })
        );
        assert!(SYMBOL.resolve().is_err());
    }

    #[test]
    fn present_symbol_resolves_to_the_linked_address() {
        static SYMBOL: Symbol = Symbol::new("VTCompressionSessionGetTypeID\0", "10.8");

        let address = SYMBOL.resolve().expect("VideoToolbox is loaded");
        let linked = crate::ffi::VTCompressionSessionGetTypeID as unsafe extern "C" fn() -> usize;

        assert_eq!(
            address.as_ptr().cast_const(),
            linked as *const core::ffi::c_void
        );
        assert_eq!(SYMBOL.resolve(), Ok(address));
    }

    #[test]
    fn present_constant_matches_the_linked_value() {
        let Ok(dynamic) = super::kVTHDRPerFrameMetadataGenerationOptionsKey_HDRFormats() else {
            return;
        };
        let linked = unsafe { crate::ffi::kVTHDRPerFrameMetadataGenerationOptionsKey_HDRFormats };

        assert!(!dynamic.is_null());
        assert_eq!(dynamic, linked);
    }
}
