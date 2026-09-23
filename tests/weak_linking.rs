use std::path::{Path, PathBuf};
use std::process::Command;

const NEWER_THAN_MACOS_13: &[&str] = &[
    "CMTaggedBufferGroupGetCMSampleBufferAtIndex",
    "CMTaggedBufferGroupGetCVPixelBufferAtIndex",
    "CMTaggedBufferGroupGetCount",
    "CMTaggedBufferGroupGetTypeID",
    "VTCompressionSessionEncodeMultiImageFrame",
    "VTCompressionSessionEncodeMultiImageFrameWithOutputHandler",
    "VTCopyRAWProcessorExtensionProperties",
    "VTCopyVideoDecoderExtensionProperties",
    "VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler",
    "VTDecompressionSessionDecodeFrameWithOptions",
    "VTDecompressionSessionDecodeFrameWithOptionsAndOutputHandler",
    "VTDecompressionSessionSetMultiImageCallback",
    "VTHDRPerFrameMetadataGenerationSessionAttachMetadata",
    "VTHDRPerFrameMetadataGenerationSessionCreate",
    "VTHDRPerFrameMetadataGenerationSessionGetTypeID",
    "VTIsStereoMVHEVCDecodeSupported",
    "VTIsStereoMVHEVCEncodeSupported",
    "VTMotionEstimationSessionCompleteFrames",
    "VTMotionEstimationSessionCopySourcePixelBufferAttributes",
    "VTMotionEstimationSessionCreate",
    "VTMotionEstimationSessionGetTypeID",
    "VTMotionEstimationSessionInvalidate",
    "VTRAWProcessingSessionCompleteFrames",
    "VTRAWProcessingSessionCopyProcessingParameters",
    "VTRAWProcessingSessionCreate",
    "VTRAWProcessingSessionGetTypeID",
    "VTRAWProcessingSessionInvalidate",
    "VTRAWProcessingSessionSetParameterChangedHandler",
    "VTRAWProcessingSessionSetProcessingParameters",
    "kVTCameraCalibrationExtrinsicOriginSource_StereoCameraSystemBaseline",
    "kVTCameraCalibrationLensAlgorithmKind_ParametricLens",
    "kVTCameraCalibrationLensDomain_Color",
    "kVTCameraCalibrationLensRole_Left",
    "kVTCameraCalibrationLensRole_Mono",
    "kVTCameraCalibrationLensRole_Right",
    "kVTCompressionPreset_Balanced",
    "kVTCompressionPreset_HighQuality",
    "kVTCompressionPreset_HighSpeed",
    "kVTCompressionPreset_VideoConferencing",
    "kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOrientationQuaternion",
    "kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOriginSource",
    "kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrix",
    "kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixProjectionOffset",
    "kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixReferenceDimensions",
    "kVTCompressionPropertyCameraCalibrationKey_LensAlgorithmKind",
    "kVTCompressionPropertyCameraCalibrationKey_LensDistortions",
    "kVTCompressionPropertyCameraCalibrationKey_LensDomain",
    "kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialX",
    "kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialY",
    "kVTCompressionPropertyCameraCalibrationKey_LensIdentifier",
    "kVTCompressionPropertyCameraCalibrationKey_LensRole",
    "kVTCompressionPropertyCameraCalibrationKey_RadialAngleLimit",
    "kVTCompressionPropertyKey_CalculateMeanSquaredError",
    "kVTCompressionPropertyKey_CameraCalibrationDataLensCollection",
    "kVTCompressionPropertyKey_HasLeftStereoEyeView",
    "kVTCompressionPropertyKey_HasRightStereoEyeView",
    "kVTCompressionPropertyKey_HeroEye",
    "kVTCompressionPropertyKey_HorizontalDisparityAdjustment",
    "kVTCompressionPropertyKey_HorizontalFieldOfView",
    "kVTCompressionPropertyKey_MVHEVCLeftAndRightViewIDs",
    "kVTCompressionPropertyKey_MVHEVCVideoLayerIDs",
    "kVTCompressionPropertyKey_MVHEVCViewIDs",
    "kVTCompressionPropertyKey_MaximumRealTimeFrameRate",
    "kVTCompressionPropertyKey_ProjectionKind",
    "kVTCompressionPropertyKey_RecommendedParallelizationLimit",
    "kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumDuration",
    "kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumFrameCount",
    "kVTCompressionPropertyKey_SpatialAdaptiveQPLevel",
    "kVTCompressionPropertyKey_StereoCameraBaseline",
    "kVTCompressionPropertyKey_SuggestedLookAheadFrameCount",
    "kVTCompressionPropertyKey_SupportedPresetDictionaries",
    "kVTCompressionPropertyKey_VBVBufferDuration",
    "kVTCompressionPropertyKey_VBVInitialDelayPercentage",
    "kVTCompressionPropertyKey_VBVMaxBitRate",
    "kVTCompressionPropertyKey_VariableBitRate",
    "kVTCompressionPropertyKey_ViewPackingKind",
    "kVTDecodeFrameOptionKey_ContentAnalyzerCropRectangle",
    "kVTDecodeFrameOptionKey_ContentAnalyzerRotation",
    "kVTDecompressionPropertyKey_AllowBitstreamToChangeFrameDimensions",
    "kVTDecompressionPropertyKey_DecoderProducesRAWOutput",
    "kVTDecompressionPropertyKey_GeneratePerFrameHDRDisplayMetadata",
    "kVTDecompressionPropertyKey_RequestRAWOutput",
    "kVTDecompressionPropertyKey_RequestedMVHEVCVideoLayerIDs",
    "kVTExtensionProperties_CodecNameKey",
    "kVTExtensionProperties_ContainingBundleNameKey",
    "kVTExtensionProperties_ContainingBundleURLKey",
    "kVTExtensionProperties_ExtensionIdentifierKey",
    "kVTExtensionProperties_ExtensionNameKey",
    "kVTExtensionProperties_ExtensionURLKey",
    "kVTHDRMetadataInsertionMode_RequestSDRRangePreservation",
    "kVTHDRPerFrameMetadataGenerationHDRFormatType_DolbyVision",
    "kVTHDRPerFrameMetadataGenerationOptionsKey_HDRFormats",
    "kVTHeroEye_Left",
    "kVTHeroEye_Right",
    "kVTMotionEstimationSessionCreationOption_Label",
    "kVTMotionEstimationSessionCreationOption_MotionVectorSize",
    "kVTMotionEstimationSessionCreationOption_UseMultiPassSearch",
    "kVTProjectionKind_Equirectangular",
    "kVTProjectionKind_HalfEquirectangular",
    "kVTProjectionKind_ParametricImmersive",
    "kVTProjectionKind_Rectilinear",
    "kVTRAWProcessingParameterListElement_Description",
    "kVTRAWProcessingParameterListElement_Label",
    "kVTRAWProcessingParameterListElement_ListElementID",
    "kVTRAWProcessingParameterValueType_Boolean",
    "kVTRAWProcessingParameterValueType_Float",
    "kVTRAWProcessingParameterValueType_Integer",
    "kVTRAWProcessingParameterValueType_List",
    "kVTRAWProcessingParameterValueType_SubGroup",
    "kVTRAWProcessingParameter_CameraValue",
    "kVTRAWProcessingParameter_CurrentValue",
    "kVTRAWProcessingParameter_Description",
    "kVTRAWProcessingParameter_Enabled",
    "kVTRAWProcessingParameter_InitialValue",
    "kVTRAWProcessingParameter_Key",
    "kVTRAWProcessingParameter_ListArray",
    "kVTRAWProcessingParameter_MaximumValue",
    "kVTRAWProcessingParameter_MinimumValue",
    "kVTRAWProcessingParameter_Name",
    "kVTRAWProcessingParameter_NeutralValue",
    "kVTRAWProcessingParameter_SubGroup",
    "kVTRAWProcessingParameter_ValueType",
    "kVTRAWProcessingPropertyKey_MetadataForSidecarFile",
    "kVTRAWProcessingPropertyKey_MetalDeviceRegistryID",
    "kVTRAWProcessingPropertyKey_OutputColorAttachments",
    "kVTSampleAttachmentKey_QualityMetrics",
    "kVTSampleAttachmentQualityMetricsKey_ChromaBlueMeanSquaredError",
    "kVTSampleAttachmentQualityMetricsKey_ChromaRedMeanSquaredError",
    "kVTSampleAttachmentQualityMetricsKey_LumaMeanSquaredError",
    "kVTViewPackingKind_OverUnder",
    "kVTViewPackingKind_SideBySide",
];

fn nm(path: &Path) -> Option<String> {
    let output = Command::new("xcrun")
        .arg("nm")
        .arg("-m")
        .arg(path)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

fn imports(listing: &str) -> Vec<(&str, bool, &str)> {
    listing
        .lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("(undefined) ")?;
            let (weak, rest) = rest.strip_prefix("weak external _").map_or_else(
                || (false, rest.strip_prefix("external _")),
                |rest| (true, Some(rest)),
            );
            let rest = rest?;
            let (name, library) = rest.split_once(' ').unwrap_or((rest, ""));
            Some((name, weak, library))
        })
        .collect()
}

fn newest_library_archive() -> Option<PathBuf> {
    let deps = std::env::current_exe().ok()?.parent()?.to_path_buf();
    std::fs::read_dir(deps)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("libvideotoolbox-") && name.ends_with(".rlib"))
        })
        .max_by_key(|path| path.metadata().and_then(|meta| meta.modified()).ok())
}

#[test]
fn library_never_strongly_imports_symbols_newer_than_macos_13() {
    let Some(archive) = newest_library_archive() else {
        return;
    };
    let Some(listing) = nm(&archive) else {
        return;
    };

    let imports = imports(&listing);
    assert!(
        imports
            .iter()
            .any(|(name, _, _)| *name == "VTCompressionSessionCreate"),
        "{} does not look like the videotoolbox archive",
        archive.display()
    );
    let strong: Vec<&str> = imports
        .iter()
        .filter(|(name, weak, _)| !weak && NEWER_THAN_MACOS_13.contains(name))
        .map(|(name, _, _)| *name)
        .collect();
    assert!(
        strong.is_empty(),
        "strong references to post-macOS 13 symbols: {strong:?}"
    );
}

#[test]
fn test_binaries_weak_link_videotoolbox_and_core_media() {
    let Ok(executable) = std::env::current_exe() else {
        return;
    };
    let Some(listing) = nm(&executable) else {
        return;
    };
    std::hint::black_box(videotoolbox::DecompressionSession::is_stereo_mvhevc_decode_supported());
    std::hint::black_box(unsafe { videotoolbox::ffi::VTCompressionSessionGetTypeID() });

    let imports = imports(&listing);
    let framework_imports: Vec<_> = imports
        .iter()
        .filter(|(_, _, library)| {
            *library == "(from VideoToolbox)" || *library == "(from CoreMedia)"
        })
        .collect();
    assert!(!framework_imports.is_empty());
    let strong: Vec<&str> = framework_imports
        .iter()
        .filter(|(_, weak, _)| !weak)
        .map(|(name, _, _)| *name)
        .collect();
    assert!(strong.is_empty(), "strong framework imports: {strong:?}");
}
