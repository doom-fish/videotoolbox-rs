use apple_cf::{
    cf::CFDictionary,
    cm::{CMFormatDescription, CMSampleBuffer},
};
#[cfg(feature = "frame_processor")]
use apple_cf::{cf::CFType, cv::CVPixelBuffer};
use videotoolbox::{
    available_video_encoder_details, available_video_encoder_details_with_options, ffi,
    supported_property_dictionary_for_encoder, Codec, CompressionSession, DecompressionSession,
    EncodedFrame, EncoderSupportedProperties, TaggedBufferGroup, VTError, VideoEncoderDetails,
    VideoEncoderListOptions,
};

const TARGET_FFI_SYMBOLS: &[&str] = &[
    "kVTAlphaChannelMode_PremultipliedAlpha",
    "kVTAlphaChannelMode_StraightAlpha",
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
    "kVTCompressionPropertyKey_AllowOpenGOP",
    "kVTCompressionPropertyKey_AllowTemporalCompression",
    "kVTCompressionPropertyKey_AlphaChannelMode",
    "kVTCompressionPropertyKey_AspectRatio16x9",
    "kVTCompressionPropertyKey_BaseLayerBitRateFraction",
    "kVTCompressionPropertyKey_BaseLayerFrameRate",
    "kVTCompressionPropertyKey_BaseLayerFrameRateFraction",
    "kVTCompressionPropertyKey_CalculateMeanSquaredError",
    "kVTCompressionPropertyKey_CameraCalibrationDataLensCollection",
    "kVTCompressionPropertyKey_CleanAperture",
    "kVTCompressionPropertyKey_ConstantBitRate",
    "kVTCompressionPropertyKey_ContentLightLevelInfo",
    "kVTCompressionPropertyKey_DataRateLimits",
    "kVTCompressionPropertyKey_Depth",
    "kVTCompressionPropertyKey_EnableLTR",
    "kVTCompressionPropertyKey_EncoderID",
    "kVTCompressionPropertyKey_EstimatedAverageBytesPerFrame",
    "kVTCompressionPropertyKey_ExpectedDuration",
    "kVTCompressionPropertyKey_FieldCount",
    "kVTCompressionPropertyKey_FieldDetail",
    "kVTCompressionPropertyKey_GammaLevel",
    "kVTCompressionPropertyKey_HDRMetadataInsertionMode",
    "kVTCompressionPropertyKey_HasLeftStereoEyeView",
    "kVTCompressionPropertyKey_HasRightStereoEyeView",
    "kVTCompressionPropertyKey_HeroEye",
    "kVTCompressionPropertyKey_HorizontalDisparityAdjustment",
    "kVTCompressionPropertyKey_HorizontalFieldOfView",
    "kVTCompressionPropertyKey_MVHEVCLeftAndRightViewIDs",
    "kVTCompressionPropertyKey_MVHEVCVideoLayerIDs",
    "kVTCompressionPropertyKey_MVHEVCViewIDs",
    "kVTCompressionPropertyKey_MasteringDisplayColorVolume",
    "kVTCompressionPropertyKey_MaxAllowedFrameQP",
    "kVTCompressionPropertyKey_MaxFrameDelayCount",
    "kVTCompressionPropertyKey_MaxH264SliceBytes",
    "kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration",
    "kVTCompressionPropertyKey_MaximizePowerEfficiency",
    "kVTCompressionPropertyKey_MaximumRealTimeFrameRate",
    "kVTCompressionPropertyKey_MinAllowedFrameQP",
    "kVTCompressionPropertyKey_MoreFramesAfterEnd",
    "kVTCompressionPropertyKey_MoreFramesBeforeStart",
    "kVTCompressionPropertyKey_NumberOfPendingFrames",
    "kVTCompressionPropertyKey_OutputBitDepth",
    "kVTCompressionPropertyKey_PixelAspectRatio",
    "kVTCompressionPropertyKey_PixelBufferPoolIsShared",
    "kVTCompressionPropertyKey_PixelTransferProperties",
    "kVTCompressionPropertyKey_PreserveAlphaChannel",
    "kVTCompressionPropertyKey_PreserveDynamicHDRMetadata",
    "kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality",
    "kVTCompressionPropertyKey_ProgressiveScan",
    "kVTCompressionPropertyKey_ProjectionKind",
    "kVTCompressionPropertyKey_RecommendedParallelizationLimit",
    "kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumDuration",
    "kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumFrameCount",
    "kVTCompressionPropertyKey_ReferenceBufferCount",
    "kVTCompressionPropertyKey_SourceFrameCount",
    "kVTCompressionPropertyKey_SpatialAdaptiveQPLevel",
    "kVTCompressionPropertyKey_StereoCameraBaseline",
    "kVTCompressionPropertyKey_SuggestedLookAheadFrameCount",
    "kVTCompressionPropertyKey_SupportedPresetDictionaries",
    "kVTCompressionPropertyKey_SupportsBaseFrameQP",
    "kVTCompressionPropertyKey_TargetQualityForAlpha",
    "kVTCompressionPropertyKey_UsingGPURegistryID",
    "kVTCompressionPropertyKey_UsingHardwareAcceleratedVideoEncoder",
    "kVTCompressionPropertyKey_VBVBufferDuration",
    "kVTCompressionPropertyKey_VBVInitialDelayPercentage",
    "kVTCompressionPropertyKey_VBVMaxBitRate",
    "kVTCompressionPropertyKey_VariableBitRate",
    "kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes",
    "kVTCompressionPropertyKey_ViewPackingKind",
    "kVTEncodeFrameOptionKey_AcknowledgedLTRTokens",
    "kVTEncodeFrameOptionKey_BaseFrameQP",
    "kVTEncodeFrameOptionKey_ForceKeyFrame",
    "kVTEncodeFrameOptionKey_ForceLTRRefresh",
    "kVTHDRMetadataInsertionMode_Auto",
    "kVTHDRMetadataInsertionMode_None",
    "kVTHDRMetadataInsertionMode_RequestSDRRangePreservation",
    "kVTHeroEye_Left",
    "kVTHeroEye_Right",
    "kVTProjectionKind_Equirectangular",
    "kVTProjectionKind_HalfEquirectangular",
    "kVTProjectionKind_ParametricImmersive",
    "kVTProjectionKind_Rectilinear",
    "kVTSampleAttachmentKey_QualityMetrics",
    "kVTSampleAttachmentKey_RequireLTRAcknowledgementToken",
    "kVTSampleAttachmentQualityMetricsKey_ChromaBlueMeanSquaredError",
    "kVTSampleAttachmentQualityMetricsKey_ChromaRedMeanSquaredError",
    "kVTSampleAttachmentQualityMetricsKey_LumaMeanSquaredError",
    "kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder",
    "kVTVideoEncoderSpecification_EnableLowLatencyRateControl",
    "kVTVideoEncoderSpecification_PreferredEncoderGPURegistryID",
    "kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder",
    "kVTVideoEncoderSpecification_RequiredEncoderGPURegistryID",
    "kVTViewPackingKind_OverUnder",
    "kVTViewPackingKind_SideBySide",
    "VTCompressionSessionEncodeFrameWithOutputHandler",
    "VTCompressionSessionEncodeMultiImageFrame",
    "VTCompressionSessionEncodeMultiImageFrameWithOutputHandler",
    "VTCompressionSessionOptionFlags",
    "VTIsStereoMVHEVCEncodeSupported",
    "kVTVideoEncoderSpecification_EncoderID",
    "kVTDecodeFrameOptionKey_ContentAnalyzerCropRectangle",
    "kVTDecodeFrameOptionKey_ContentAnalyzerRotation",
    "kVTDecompressionPropertyKey_AllowBitstreamToChangeFrameDimensions",
    "kVTDecompressionPropertyKey_ContentHasInterframeDependencies",
    "kVTDecompressionPropertyKey_DecoderProducesRAWOutput",
    "kVTDecompressionPropertyKey_DeinterlaceMode",
    "kVTDecompressionPropertyKey_FieldMode",
    "kVTDecompressionPropertyKey_GeneratePerFrameHDRDisplayMetadata",
    "kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded",
    "kVTDecompressionPropertyKey_MaximizePowerEfficiency",
    "kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded",
    "kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded",
    "kVTDecompressionPropertyKey_OnlyTheseFrames",
    "kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount",
    "kVTDecompressionPropertyKey_PixelBufferPool",
    "kVTDecompressionPropertyKey_PixelBufferPoolIsShared",
    "kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport",
    "kVTDecompressionPropertyKey_PixelTransferProperties",
    "kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata",
    "kVTDecompressionPropertyKey_ReducedCoefficientDecode",
    "kVTDecompressionPropertyKey_ReducedFrameDelivery",
    "kVTDecompressionPropertyKey_ReducedResolutionDecode",
    "kVTDecompressionPropertyKey_RequestRAWOutput",
    "kVTDecompressionPropertyKey_RequestedMVHEVCVideoLayerIDs",
    "kVTDecompressionPropertyKey_SuggestedQualityOfServiceTiers",
    "kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance",
    "kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByQuality",
    "kVTDecompressionPropertyKey_ThreadCount",
    "kVTDecompressionPropertyKey_UsingGPURegistryID",
    "kVTDecompressionProperty_DeinterlaceMode_Temporal",
    "kVTDecompressionProperty_DeinterlaceMode_VerticalFilter",
    "kVTDecompressionProperty_FieldMode_BothFields",
    "kVTDecompressionProperty_FieldMode_BottomFieldOnly",
    "kVTDecompressionProperty_FieldMode_DeinterlaceFields",
    "kVTDecompressionProperty_FieldMode_SingleField",
    "kVTDecompressionProperty_FieldMode_TopFieldOnly",
    "kVTDecompressionProperty_OnlyTheseFrames_AllFrames",
    "kVTDecompressionProperty_OnlyTheseFrames_IFrames",
    "kVTDecompressionProperty_OnlyTheseFrames_KeyFrames",
    "kVTDecompressionProperty_OnlyTheseFrames_NonDroppableFrames",
    "kVTDecompressionProperty_TemporalLevelLimit",
    "kVTDecompressionResolutionKey_Height",
    "kVTDecompressionResolutionKey_Width",
    "kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder",
    "kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID",
    "kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder",
    "kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID",
    "VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler",
    "VTDecompressionSessionDecodeFrameWithOptions",
    "VTDecompressionSessionDecodeFrameWithOptionsAndOutputHandler",
    "VTDecompressionSessionDecodeFrameWithOutputHandler",
    "VTDecompressionSessionSetMultiImageCallback",
    "VTIsStereoMVHEVCDecodeSupported",
    "VTDecodeFrameFlags",
    "VTDecodeInfoFlags",
    "VTCopySupportedPropertyDictionaryForEncoder",
    "kVTVideoEncoderListOption_IncludeStandardDefinitionDVEncoders",
    "kVTVideoEncoderList_GPURegistryID",
    "kVTVideoEncoderList_InstanceLimit",
    "kVTVideoEncoderList_IsHardwareAccelerated",
    "kVTVideoEncoderList_PerformanceRating",
    "kVTVideoEncoderList_QualityRating",
    "kVTVideoEncoderList_SupportedSelectionProperties",
    "kVTVideoEncoderList_SupportsFrameReordering",
];

const TARGET_FFI_SYMBOLS_V0_11_2: &[&str] = &[
    "VTFrameRateConversionConfigurationQualityPrioritization",
    "VTFrameRateConversionConfigurationRevision",
    "VTMotionBlurConfigurationQualityPrioritization",
    "VTMotionBlurConfigurationRevision",
    "VTOpticalFlowConfigurationQualityPrioritization",
    "VTOpticalFlowConfigurationRevision",
    "VTSuperResolutionScalerConfigurationInputType",
    "VTSuperResolutionScalerConfigurationQualityPrioritization",
    "VTSuperResolutionScalerConfigurationRevision",
    "VTFrameSiloCallBlockForEachSampleBuffer",
    "VTHDRPerFrameMetadataGenerationSessionGetTypeID",
    "kVTHDRPerFrameMetadataGenerationHDRFormatType_DolbyVision",
    "kVTHDRPerFrameMetadataGenerationOptionsKey_HDRFormats",
    "VTMotionEstimationFrameFlags",
    "VTMotionEstimationInfoFlags",
    "kVTMotionEstimationSessionCreationOption_Label",
    "kVTMotionEstimationSessionCreationOption_MotionVectorSize",
    "kVTMotionEstimationSessionCreationOption_UseMultiPassSearch",
    "kVTRAWProcessingPropertyKey_MetadataForSidecarFile",
    "kVTRAWProcessingPropertyKey_MetalDeviceRegistryID",
    "kVTRAWProcessingPropertyKey_OutputColorAttachments",
    "VTRAWProcessingSessionSetParameterChangedHandler",
    "kVTPropertyDocumentationKey",
    "kVTPropertyReadWriteStatusKey",
    "kVTPropertyReadWriteStatus_ReadOnly",
    "kVTPropertyReadWriteStatus_ReadWrite",
    "kVTPropertyShouldBeSerializedKey",
    "kVTPropertySupportedValueListKey",
    "kVTPropertySupportedValueMaximumKey",
    "kVTPropertySupportedValueMinimumKey",
    "kVTPropertyTypeKey",
    "kVTPropertyType_Boolean",
    "kVTPropertyType_Enumeration",
    "kVTPropertyType_Number",
    "VTCopyRAWProcessorExtensionProperties",
    "VTCopyVideoDecoderExtensionProperties",
    "VTRegisterSupplementalVideoDecoderIfAvailable",
    "kVTExtensionProperties_CodecNameKey",
    "kVTExtensionProperties_ContainingBundleNameKey",
    "kVTExtensionProperties_ContainingBundleURLKey",
    "kVTExtensionProperties_ExtensionIdentifierKey",
    "kVTExtensionProperties_ExtensionNameKey",
    "kVTExtensionProperties_ExtensionURLKey",
];

#[test]
fn targeted_ffi_gap_symbols_are_present() {
    let ffi_src = include_str!("../src/ffi/mod.rs");
    for symbol in TARGET_FFI_SYMBOLS
        .iter()
        .chain(TARGET_FFI_SYMBOLS_V0_11_2.iter())
    {
        assert!(
            ffi_src.contains(symbol),
            "{symbol} missing from src/ffi/mod.rs"
        );
    }
}

type EncodeMultiImageFn =
    fn(&CompressionSession, &TaggedBufferGroup, (i64, i32)) -> Result<EncodedFrame, VTError>;
type DecodeWithOptionsFn = fn(
    &DecompressionSession,
    &CMSampleBuffer,
    ffi::VTDecodeFrameFlags,
    Option<&CFDictionary>,
) -> Result<ffi::VTDecodeInfoFlags, VTError>;
type SetMultiImageCallbackFn =
    fn(&DecompressionSession, fn(videotoolbox::DecodedMultiImageFrame)) -> Result<(), VTError>;
type EncoderDetailsFn = fn() -> Result<Vec<VideoEncoderDetails>, i32>;
type EncoderDetailsWithOptionsFn =
    fn(&VideoEncoderListOptions) -> Result<Vec<VideoEncoderDetails>, i32>;
type SupportedPropertyDictionaryFn =
    fn(i32, i32, Codec, Option<&str>) -> Result<EncoderSupportedProperties, i32>;

#[test]
fn new_safe_wrappers_are_reachable() {
    let encode_multi_image_fn: EncodeMultiImageFn = CompressionSession::encode_multi_image;
    let compression_invalidate_fn: fn(CompressionSession) -> Result<(), VTError> =
        CompressionSession::invalidate;
    let stereo_encode_support_fn: fn() -> bool =
        CompressionSession::is_stereo_mvhevc_encode_supported;
    let decompression_invalidate_fn: fn(DecompressionSession) -> Result<(), VTError> =
        DecompressionSession::invalidate;
    let stereo_decode_support_fn: fn() -> bool =
        DecompressionSession::is_stereo_mvhevc_decode_supported;
    let decode_with_options_fn: DecodeWithOptionsFn = DecompressionSession::decode_with_options;
    let set_multi_image_callback_fn: SetMultiImageCallbackFn =
        DecompressionSession::set_multi_image_callback::<fn(videotoolbox::DecodedMultiImageFrame)>;
    let encoder_details_fn: EncoderDetailsFn = available_video_encoder_details;
    let encoder_details_with_options_fn: EncoderDetailsWithOptionsFn =
        available_video_encoder_details_with_options;
    let supported_property_dictionary_fn: SupportedPropertyDictionaryFn =
        supported_property_dictionary_for_encoder;
    let tagged_buffer_group_type_id_fn: fn() -> usize = TaggedBufferGroup::type_id;
    let tagged_buffer_group_len_fn: fn(&TaggedBufferGroup) -> usize = TaggedBufferGroup::len;
    let hdr_type_id_fn: fn() -> usize = videotoolbox::hdr_metadata::HdrMetadataSession::type_id;
    let hdr_new_with_formats_fn: fn(
        f32,
        &[videotoolbox::hdr_metadata::HdrMetadataFormat],
    ) -> Result<
        videotoolbox::hdr_metadata::HdrMetadataSession,
        VTError,
    > = videotoolbox::hdr_metadata::HdrMetadataSession::new_with_formats;
    let register_supplemental_decoder_fn: fn(Codec) =
        videotoolbox::utilities::register_supplemental_video_decoder_if_available;
    let copy_decoder_extension_properties_fn: fn(
        &CMFormatDescription,
    ) -> Result<CFDictionary, VTError> =
        videotoolbox::utilities::copy_video_decoder_extension_properties;
    let copy_raw_processor_extension_properties_fn: fn(
        &CMFormatDescription,
    ) -> Result<CFDictionary, VTError> =
        videotoolbox::utilities::copy_raw_processor_extension_properties;
    let _ = videotoolbox::hdr_metadata::HdrMetadataFormat::DolbyVision;

    let _ = (
        encode_multi_image_fn,
        compression_invalidate_fn,
        stereo_encode_support_fn,
        decompression_invalidate_fn,
        stereo_decode_support_fn,
        decode_with_options_fn,
        set_multi_image_callback_fn,
        encoder_details_fn,
        encoder_details_with_options_fn,
        supported_property_dictionary_fn,
        tagged_buffer_group_type_id_fn,
        tagged_buffer_group_len_fn,
        hdr_type_id_fn,
        hdr_new_with_formats_fn,
        register_supplemental_decoder_fn,
        copy_decoder_extension_properties_fn,
        copy_raw_processor_extension_properties_fn,
    );
}

#[cfg(feature = "frame_processor")]
#[test]
#[allow(clippy::too_many_lines)]
fn new_frame_processor_safe_wrappers_are_reachable() {
    type MotionNewWithOptionsFn =
        fn(
            u32,
            u32,
            &videotoolbox::motion_estimation::MotionEstimationSessionOptions,
        ) -> Result<videotoolbox::motion_estimation::MotionEstimationSession, VTError>;
    type MotionEstimateWithOptionsFn =
        fn(
            &videotoolbox::motion_estimation::MotionEstimationSession,
            &CVPixelBuffer,
            &CVPixelBuffer,
            ffi::VTMotionEstimationFrameFlags,
        ) -> Result<videotoolbox::motion_estimation::MotionEstimationResult, VTError>;
    type RawSetParameterChangedHandlerFn = fn(
        &videotoolbox::raw_processing::RawProcessingSession,
        fn(Vec<videotoolbox::raw_processing::RawProcessingParameter>),
    ) -> Result<(), VTError>;

    let _ = videotoolbox::frame_processor::VTFrameRateConversionConfigurationQualityPrioritization::Normal;
    let _ = videotoolbox::frame_processor::VTFrameRateConversionConfigurationRevision::Revision1;
    let _ = videotoolbox::frame_processor::VTMotionBlurConfigurationQualityPrioritization::Normal;
    let _ = videotoolbox::frame_processor::VTMotionBlurConfigurationRevision::Revision1;
    let _ = videotoolbox::frame_processor::VTOpticalFlowConfigurationQualityPrioritization::Normal;
    let _ = videotoolbox::frame_processor::VTOpticalFlowConfigurationRevision::Revision1;
    let _ = videotoolbox::frame_processor::VTSuperResolutionScalerConfigurationInputType::Video;
    let _ = videotoolbox::frame_processor::VTSuperResolutionScalerConfigurationQualityPrioritization::Normal;
    let _ = videotoolbox::frame_processor::VTSuperResolutionScalerConfigurationRevision::Revision1;
    let _ = videotoolbox::frame_processor::SuperResolutionConfiguration::new(1920, 1080, 2);
    let _ = videotoolbox::frame_processor::MotionBlurConfiguration::new(1920, 1080);
    let _ = videotoolbox::frame_processor::FrameRateConversionConfiguration::new(1920, 1080);
    let _ = videotoolbox::frame_processor::OpticalFlowConfiguration::new(1920, 1080);
    let _motion_estimation_options =
        videotoolbox::motion_estimation::MotionEstimationSessionOptions::default();

    let super_resolution_model_status_fn: fn(
        videotoolbox::frame_processor::SuperResolutionConfiguration,
    ) -> Option<
        videotoolbox::frame_processor::SuperResolutionModelStatus,
    > = videotoolbox::frame_processor::super_resolution_model_status_for_configuration;
    let super_resolution_model_percentage_available_fn: fn(
        videotoolbox::frame_processor::SuperResolutionConfiguration,
    ) -> Option<f32> =
        videotoolbox::frame_processor::super_resolution_model_percentage_available_for_configuration;
    let download_super_resolution_model_fn: fn(
        videotoolbox::frame_processor::SuperResolutionConfiguration,
    ) -> Result<(), VTError> =
        videotoolbox::frame_processor::download_super_resolution_model_for_configuration;
    let start_super_resolution_fn: fn(
        videotoolbox::frame_processor::SuperResolutionConfiguration,
    ) -> Result<
        videotoolbox::frame_processor::FrameProcessor,
        VTError,
    > = videotoolbox::frame_processor::FrameProcessor::start_super_resolution_with_configuration;
    let start_motion_blur_fn: fn(
        videotoolbox::frame_processor::MotionBlurConfiguration,
    )
        -> Result<videotoolbox::frame_processor::FrameProcessor, VTError> =
        videotoolbox::frame_processor::FrameProcessor::start_motion_blur_with_configuration;
    let start_frame_rate_conversion_fn: fn(
        videotoolbox::frame_processor::FrameRateConversionConfiguration,
    ) -> Result<videotoolbox::frame_processor::FrameProcessor, VTError> =
        videotoolbox::frame_processor::FrameProcessor::start_frame_rate_conversion_with_configuration;
    let start_optical_flow_fn: fn(
        videotoolbox::frame_processor::OpticalFlowConfiguration,
    ) -> Result<
        videotoolbox::frame_processor::FrameProcessor,
        VTError,
    > = videotoolbox::frame_processor::FrameProcessor::start_optical_flow_with_configuration;
    let motion_type_id_fn: fn() -> usize =
        videotoolbox::motion_estimation::MotionEstimationSession::type_id;
    let motion_new_with_options_fn: MotionNewWithOptionsFn =
        videotoolbox::motion_estimation::MotionEstimationSession::new_with_options;
    let motion_estimate_with_options_fn: MotionEstimateWithOptionsFn =
        videotoolbox::motion_estimation::MotionEstimationSession::estimate_with_options;
    let raw_type_id_fn: fn() -> usize = videotoolbox::raw_processing::RawProcessingSession::type_id;
    let raw_metadata_for_sidecar_file_fn: fn(
        &videotoolbox::raw_processing::RawProcessingSession,
    ) -> Result<Option<CFType>, VTError> =
        videotoolbox::raw_processing::RawProcessingSession::metadata_for_sidecar_file;
    let raw_metal_device_registry_id_fn: fn(
        &videotoolbox::raw_processing::RawProcessingSession,
    ) -> Result<Option<CFType>, VTError> =
        videotoolbox::raw_processing::RawProcessingSession::metal_device_registry_id;
    let raw_output_color_attachments_fn: fn(
        &videotoolbox::raw_processing::RawProcessingSession,
    ) -> Result<Option<CFType>, VTError> =
        videotoolbox::raw_processing::RawProcessingSession::output_color_attachments;
    let raw_set_parameter_changed_handler_fn: RawSetParameterChangedHandlerFn =
        videotoolbox::raw_processing::RawProcessingSession::set_parameter_changed_handler::<
            fn(Vec<videotoolbox::raw_processing::RawProcessingParameter>),
        >;
    let raw_clear_parameter_changed_handler_fn: fn(
        &videotoolbox::raw_processing::RawProcessingSession,
    ) -> Result<(), VTError> =
        videotoolbox::raw_processing::RawProcessingSession::clear_parameter_changed_handler;

    let _ = (
        super_resolution_model_status_fn,
        super_resolution_model_percentage_available_fn,
        download_super_resolution_model_fn,
        start_super_resolution_fn,
        start_motion_blur_fn,
        start_frame_rate_conversion_fn,
        start_optical_flow_fn,
        motion_type_id_fn,
        motion_new_with_options_fn,
        motion_estimate_with_options_fn,
        raw_type_id_fn,
        raw_metadata_for_sidecar_file_fn,
        raw_metal_device_registry_id_fn,
        raw_output_color_attachments_fn,
        raw_set_parameter_changed_handler_fn,
        raw_clear_parameter_changed_handler_fn,
    );
}
