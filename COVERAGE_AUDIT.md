# videotoolbox-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 449
VERIFIED: 216
GAPS: 232
EXEMPT: 1
COVERAGE_PCT: 48.21%

Audit scope: top-level public VideoToolbox symbols from the macOS SDK headers only (interfaces/protocols, typedef enum/struct/opaque refs, exported constants, and exported functions). Exact-name matches in the public `ffi` module count as verified; ObjC-only frame-processor, motion-estimation, and RAW-processing entry points are credited when the public safe wrapper reaches them through the Swift bridge.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `kVTCompressionPropertyKey_AllowFrameReordering` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_AllowFrameReordering` |
| `kVTCompressionPropertyKey_AverageBitRate` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_AverageBitRate` |
| `kVTCompressionPropertyKey_ColorPrimaries` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ColorPrimaries` |
| `kVTCompressionPropertyKey_ExpectedFrameRate` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ExpectedFrameRate` |
| `kVTCompressionPropertyKey_H264EntropyMode` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_H264EntropyMode` |
| `kVTCompressionPropertyKey_ICCProfile` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ICCProfile` |
| `kVTCompressionPropertyKey_MaxKeyFrameInterval` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MaxKeyFrameInterval` |
| `kVTCompressionPropertyKey_MultiPassStorage` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MultiPassStorage` |
| `kVTCompressionPropertyKey_ProfileLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ProfileLevel` |
| `kVTCompressionPropertyKey_Quality` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_Quality` |
| `kVTCompressionPropertyKey_RealTime` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_RealTime` |
| `kVTCompressionPropertyKey_TransferFunction` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_TransferFunction` |
| `kVTCompressionPropertyKey_YCbCrMatrix` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_YCbCrMatrix` |
| `kVTH264EntropyMode_CABAC` | constant | `VTCompressionProperties.h` | `ffi::kVTH264EntropyMode_CABAC` |
| `kVTH264EntropyMode_CAVLC` | constant | `VTCompressionProperties.h` | `ffi::kVTH264EntropyMode_CAVLC` |
| `kVTProfileLevel_H263_Profile0_Level10` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H263_Profile0_Level10` |
| `kVTProfileLevel_H263_Profile0_Level45` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H263_Profile0_Level45` |
| `kVTProfileLevel_H263_Profile3_Level45` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H263_Profile3_Level45` |
| `kVTProfileLevel_H264_Baseline_1_3` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_1_3` |
| `kVTProfileLevel_H264_Baseline_3_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_3_0` |
| `kVTProfileLevel_H264_Baseline_3_1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_3_1` |
| `kVTProfileLevel_H264_Baseline_3_2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_3_2` |
| `kVTProfileLevel_H264_Baseline_4_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_4_0` |
| `kVTProfileLevel_H264_Baseline_4_1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_4_1` |
| `kVTProfileLevel_H264_Baseline_4_2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_4_2` |
| `kVTProfileLevel_H264_Baseline_5_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_5_0` |
| `kVTProfileLevel_H264_Baseline_5_1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_5_1` |
| `kVTProfileLevel_H264_Baseline_5_2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_5_2` |
| `kVTProfileLevel_H264_Baseline_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Baseline_AutoLevel` |
| `kVTProfileLevel_H264_ConstrainedBaseline_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_ConstrainedBaseline_AutoLevel` |
| `kVTProfileLevel_H264_ConstrainedHigh_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_ConstrainedHigh_AutoLevel` |
| `kVTProfileLevel_H264_Extended_5_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Extended_5_0` |
| `kVTProfileLevel_H264_Extended_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Extended_AutoLevel` |
| `kVTProfileLevel_H264_High_3_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_3_0` |
| `kVTProfileLevel_H264_High_3_1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_3_1` |
| `kVTProfileLevel_H264_High_3_2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_3_2` |
| `kVTProfileLevel_H264_High_4_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_4_0` |
| `kVTProfileLevel_H264_High_4_1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_4_1` |
| `kVTProfileLevel_H264_High_4_2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_4_2` |
| `kVTProfileLevel_H264_High_5_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_5_0` |
| `kVTProfileLevel_H264_High_5_1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_5_1` |
| `kVTProfileLevel_H264_High_5_2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_5_2` |
| `kVTProfileLevel_H264_High_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_High_AutoLevel` |
| `kVTProfileLevel_H264_Main_3_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_3_0` |
| `kVTProfileLevel_H264_Main_3_1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_3_1` |
| `kVTProfileLevel_H264_Main_3_2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_3_2` |
| `kVTProfileLevel_H264_Main_4_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_4_0` |
| `kVTProfileLevel_H264_Main_4_1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_4_1` |
| `kVTProfileLevel_H264_Main_4_2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_4_2` |
| `kVTProfileLevel_H264_Main_5_0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_5_0` |
| `kVTProfileLevel_H264_Main_5_1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_5_1` |
| `kVTProfileLevel_H264_Main_5_2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_5_2` |
| `kVTProfileLevel_H264_Main_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_H264_Main_AutoLevel` |
| `kVTProfileLevel_HEVC_Main10_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_HEVC_Main10_AutoLevel` |
| `kVTProfileLevel_HEVC_Main42210_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_HEVC_Main42210_AutoLevel` |
| `kVTProfileLevel_HEVC_Main_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_HEVC_Main_AutoLevel` |
| `kVTProfileLevel_HEVC_Monochrome10_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_HEVC_Monochrome10_AutoLevel` |
| `kVTProfileLevel_HEVC_Monochrome_AutoLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_HEVC_Monochrome_AutoLevel` |
| `kVTProfileLevel_MP4V_AdvancedSimple_L0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_AdvancedSimple_L0` |
| `kVTProfileLevel_MP4V_AdvancedSimple_L1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_AdvancedSimple_L1` |
| `kVTProfileLevel_MP4V_AdvancedSimple_L2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_AdvancedSimple_L2` |
| `kVTProfileLevel_MP4V_AdvancedSimple_L3` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_AdvancedSimple_L3` |
| `kVTProfileLevel_MP4V_AdvancedSimple_L4` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_AdvancedSimple_L4` |
| `kVTProfileLevel_MP4V_Main_L2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_Main_L2` |
| `kVTProfileLevel_MP4V_Main_L3` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_Main_L3` |
| `kVTProfileLevel_MP4V_Main_L4` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_Main_L4` |
| `kVTProfileLevel_MP4V_Simple_L0` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_Simple_L0` |
| `kVTProfileLevel_MP4V_Simple_L1` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_Simple_L1` |
| `kVTProfileLevel_MP4V_Simple_L2` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_Simple_L2` |
| `kVTProfileLevel_MP4V_Simple_L3` | constant | `VTCompressionProperties.h` | `ffi::kVTProfileLevel_MP4V_Simple_L3` |
| `VTCompressionSessionBeginPass` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionBeginPass` |
| `VTCompressionSessionCompleteFrames` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionCompleteFrames` |
| `VTCompressionSessionCreate` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionCreate` |
| `VTCompressionSessionEncodeFrame` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionEncodeFrame` |
| `VTCompressionSessionEndPass` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionEndPass` |
| `VTCompressionSessionGetPixelBufferPool` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionGetPixelBufferPool` |
| `VTCompressionSessionGetTimeRangesForNextPass` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionGetTimeRangesForNextPass` |
| `VTCompressionSessionGetTypeID` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionGetTypeID` |
| `VTCompressionSessionInvalidate` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionInvalidate` |
| `VTCompressionSessionPrepareToEncodeFrames` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionPrepareToEncodeFrames` |
| `VTCompressionSessionRef` | opaque ref | `VTCompressionSession.h` | `ffi::VTCompressionSessionRef` |
| `kVTDecompressionPropertyKey_RealTime` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_RealTime` |
| `kVTDecompressionPropertyKey_UsingHardwareAcceleratedVideoDecoder` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_UsingHardwareAcceleratedVideoDecoder` |
| `VTDecompressionOutputCallbackRecord` | struct | `VTDecompressionSession.h` | `ffi::VTDecompressionOutputCallbackRecord` |
| `VTDecompressionSessionCanAcceptFormatDescription` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionCanAcceptFormatDescription` |
| `VTDecompressionSessionCopyBlackPixelBuffer` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionCopyBlackPixelBuffer` |
| `VTDecompressionSessionCreate` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionCreate` |
| `VTDecompressionSessionDecodeFrame` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionDecodeFrame` |
| `VTDecompressionSessionFinishDelayedFrames` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionFinishDelayedFrames` |
| `VTDecompressionSessionGetTypeID` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionGetTypeID` |
| `VTDecompressionSessionInvalidate` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionInvalidate` |
| `VTDecompressionSessionRef` | opaque ref | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionRef` |
| `VTDecompressionSessionWaitForAsynchronousFrames` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionWaitForAsynchronousFrames` |
| `VTIsHardwareDecodeSupported` | function | `VTDecompressionSession.h` | `ffi::VTIsHardwareDecodeSupported` |
| `VTEncodeInfoFlags` | options | `VTErrors.h` | `ffi::VTEncodeInfoFlags` |
| `VTFrameProcessor` | interface | `VTFrameProcessor.h` | `FrameProcessor via swift-bridge/Sources/VideoToolboxBridge/FrameProcessor.swift` |
| `VTFrameProcessorConfiguration` | protocol | `VTFrameProcessorConfiguration.h` | `frame_processor::frame_processor_capabilities / FrameProcessor::start_* via swift-bridge` |
| `VTFrameProcessorFrame` | interface | `VTFrameProcessorFrame.h` | `FrameProcessorFrame via vt_frame_processor_frame_create` |
| `VTFrameProcessorOpticalFlow` | interface | `VTFrameProcessorFrame.h` | `FrameProcessorOpticalFlow via vt_frame_processor_optical_flow_create` |
| `VTFrameProcessorParameters` | protocol | `VTFrameProcessorParameters.h` | `FrameProcessor::process_* helpers via swift-bridge` |
| `VTFrameRateConversionConfiguration` | interface | `VTFrameProcessor_FrameRateConversion.h` | `FrameProcessor::start_frame_rate_conversion + frame_processor_capabilities` |
| `VTFrameRateConversionParameters` | interface | `VTFrameProcessor_FrameRateConversion.h` | `FrameProcessor::process_frame_rate_conversion / _with_command_buffer` |
| `VTFrameRateConversionParametersSubmissionMode` | enum | `VTFrameProcessor_FrameRateConversion.h` | `FrameRateConversionSubmissionMode` |
| `VTLowLatencyFrameInterpolationConfiguration` | interface | `VTFrameProcessor_LowLatencyFrameInterpolation.h` | `FrameProcessor::start_low_latency_frame_interpolation + frame_processor_capabilities` |
| `VTLowLatencyFrameInterpolationParameters` | interface | `VTFrameProcessor_LowLatencyFrameInterpolation.h` | `FrameProcessor::process_low_latency_frame_interpolation / _with_command_buffer` |
| `VTLowLatencySuperResolutionScalerConfiguration` | interface | `VTFrameProcessor_LowLatencySuperResolutionScaler.h` | `FrameProcessor::start_low_latency_super_resolution + supported_scale_factors` |
| `VTLowLatencySuperResolutionScalerParameters` | interface | `VTFrameProcessor_LowLatencySuperResolutionScaler.h` | `FrameProcessor::process_low_latency_super_resolution / _with_command_buffer` |
| `VTMotionBlurConfiguration` | interface | `VTFrameProcessor_MotionBlur.h` | `FrameProcessor::start_motion_blur + frame_processor_capabilities` |
| `VTMotionBlurParameters` | interface | `VTFrameProcessor_MotionBlur.h` | `FrameProcessor::process_motion_blur / _with_command_buffer` |
| `VTMotionBlurParametersSubmissionMode` | enum | `VTFrameProcessor_MotionBlur.h` | `FrameProcessorSubmissionMode` |
| `VTOpticalFlowConfiguration` | interface | `VTFrameProcessor_OpticalFlow.h` | `FrameProcessor::start_optical_flow + frame_processor_capabilities` |
| `VTOpticalFlowParameters` | interface | `VTFrameProcessor_OpticalFlow.h` | `FrameProcessor::process_optical_flow / _with_command_buffer` |
| `VTOpticalFlowParametersSubmissionMode` | enum | `VTFrameProcessor_OpticalFlow.h` | `FrameProcessorSubmissionMode` |
| `VTSuperResolutionScalerConfiguration` | interface | `VTFrameProcessor_SuperResolutionScaler.h` | `FrameProcessor::start_super_resolution + frame_processor_capabilities` |
| `VTSuperResolutionScalerConfigurationModelStatus` | enum | `VTFrameProcessor_SuperResolutionScaler.h` | `SuperResolutionModelStatus` |
| `VTSuperResolutionScalerParameters` | interface | `VTFrameProcessor_SuperResolutionScaler.h` | `FrameProcessor::process_super_resolution / _with_command_buffer` |
| `VTSuperResolutionScalerParametersSubmissionMode` | enum | `VTFrameProcessor_SuperResolutionScaler.h` | `FrameProcessorSubmissionMode` |
| `VTTemporalNoiseFilterConfiguration` | interface | `VTFrameProcessor_TemporalNoiseFilter.h` | `FrameProcessor::start_temporal_noise_filter + frame_processor_capabilities` |
| `VTTemporalNoiseFilterParameters` | interface | `VTFrameProcessor_TemporalNoiseFilter.h` | `FrameProcessor::process_temporal_noise_filter / _with_command_buffer` |
| `VTFrameSiloAddSampleBuffer` | function | `VTFrameSilo.h` | `ffi::VTFrameSiloAddSampleBuffer` |
| `VTFrameSiloCallFunctionForEachSampleBuffer` | function | `VTFrameSilo.h` | `ffi::VTFrameSiloCallFunctionForEachSampleBuffer` |
| `VTFrameSiloCreate` | function | `VTFrameSilo.h` | `ffi::VTFrameSiloCreate` |
| `VTFrameSiloGetProgressOfCurrentPass` | function | `VTFrameSilo.h` | `ffi::VTFrameSiloGetProgressOfCurrentPass` |
| `VTFrameSiloGetTypeID` | function | `VTFrameSilo.h` | `ffi::VTFrameSiloGetTypeID` |
| `VTFrameSiloRef` | opaque ref | `VTFrameSilo.h` | `ffi::VTFrameSiloRef` |
| `VTFrameSiloSetTimeRangesForNextPass` | function | `VTFrameSilo.h` | `ffi::VTFrameSiloSetTimeRangesForNextPass` |
| `VTHDRPerFrameMetadataGenerationSessionAttachMetadata` | function | `VTHDRPerFrameMetadataGenerationSession.h` | `ffi::VTHDRPerFrameMetadataGenerationSessionAttachMetadata` |
| `VTHDRPerFrameMetadataGenerationSessionCreate` | function | `VTHDRPerFrameMetadataGenerationSession.h` | `ffi::VTHDRPerFrameMetadataGenerationSessionCreate` |
| `VTHDRPerFrameMetadataGenerationSessionRef` | opaque ref | `VTHDRPerFrameMetadataGenerationSession.h` | `ffi::VTHDRPerFrameMetadataGenerationSessionRef` |
| `VTMotionEstimationSessionCompleteFrames` | function | `VTMotionEstimationSession.h` | `ffi::VTMotionEstimationSessionCompleteFrames` |
| `VTMotionEstimationSessionCopySourcePixelBufferAttributes` | function | `VTMotionEstimationSession.h` | `ffi::VTMotionEstimationSessionCopySourcePixelBufferAttributes` |
| `VTMotionEstimationSessionCreate` | function | `VTMotionEstimationSession.h` | `ffi::VTMotionEstimationSessionCreate` |
| `VTMotionEstimationSessionEstimateMotionVectors` | function | `VTMotionEstimationSession.h` | `MotionEstimationSession::estimate via swift-bridge/Sources/VideoToolboxBridge/MotionEstimation.swift` |
| `VTMotionEstimationSessionGetTypeID` | function | `VTMotionEstimationSession.h` | `ffi::VTMotionEstimationSessionGetTypeID` |
| `VTMotionEstimationSessionInvalidate` | function | `VTMotionEstimationSession.h` | `ffi::VTMotionEstimationSessionInvalidate` |
| `VTMotionEstimationSessionRef` | opaque ref | `VTMotionEstimationSession.h` | `ffi::VTMotionEstimationSessionRef` |
| `VTMultiPassStorageClose` | function | `VTMultiPassStorage.h` | `ffi::VTMultiPassStorageClose` |
| `VTMultiPassStorageCreate` | function | `VTMultiPassStorage.h` | `ffi::VTMultiPassStorageCreate` |
| `VTMultiPassStorageGetTypeID` | function | `VTMultiPassStorage.h` | `ffi::VTMultiPassStorageGetTypeID` |
| `VTMultiPassStorageRef` | opaque ref | `VTMultiPassStorage.h` | `ffi::VTMultiPassStorageRef` |
| `kVTMultiPassStorageCreationOption_DoNotDelete` | constant | `VTMultiPassStorage.h` | `ffi::kVTMultiPassStorageCreationOption_DoNotDelete` |
| `kVTPixelRotationPropertyKey_FlipHorizontalOrientation` | constant | `VTPixelRotationProperties.h` | `ffi::kVTPixelRotationPropertyKey_FlipHorizontalOrientation` |
| `kVTPixelRotationPropertyKey_FlipVerticalOrientation` | constant | `VTPixelRotationProperties.h` | `ffi::kVTPixelRotationPropertyKey_FlipVerticalOrientation` |
| `kVTPixelRotationPropertyKey_Rotation` | constant | `VTPixelRotationProperties.h` | `ffi::kVTPixelRotationPropertyKey_Rotation` |
| `kVTRotation_0` | constant | `VTPixelRotationProperties.h` | `ffi::kVTRotation_0` |
| `kVTRotation_180` | constant | `VTPixelRotationProperties.h` | `ffi::kVTRotation_180` |
| `kVTRotation_CCW90` | constant | `VTPixelRotationProperties.h` | `ffi::kVTRotation_CCW90` |
| `kVTRotation_CW90` | constant | `VTPixelRotationProperties.h` | `ffi::kVTRotation_CW90` |
| `VTPixelRotationSessionCreate` | function | `VTPixelRotationSession.h` | `ffi::VTPixelRotationSessionCreate` |
| `VTPixelRotationSessionGetTypeID` | function | `VTPixelRotationSession.h` | `ffi::VTPixelRotationSessionGetTypeID` |
| `VTPixelRotationSessionInvalidate` | function | `VTPixelRotationSession.h` | `ffi::VTPixelRotationSessionInvalidate` |
| `VTPixelRotationSessionRef` | opaque ref | `VTPixelRotationSession.h` | `ffi::VTPixelRotationSessionRef` |
| `VTPixelRotationSessionRotateImage` | function | `VTPixelRotationSession.h` | `ffi::VTPixelRotationSessionRotateImage` |
| `kVTDownsamplingMode_Average` | constant | `VTPixelTransferProperties.h` | `ffi::kVTDownsamplingMode_Average` |
| `kVTDownsamplingMode_Decimate` | constant | `VTPixelTransferProperties.h` | `ffi::kVTDownsamplingMode_Decimate` |
| `kVTPixelTransferPropertyKey_DestinationCleanAperture` | constant | `VTPixelTransferProperties.h` | `ffi::kVTPixelTransferPropertyKey_DestinationCleanAperture` |
| `kVTPixelTransferPropertyKey_DestinationColorPrimaries` | constant | `VTPixelTransferProperties.h` | `ffi::kVTPixelTransferPropertyKey_DestinationColorPrimaries` |
| `kVTPixelTransferPropertyKey_DestinationICCProfile` | constant | `VTPixelTransferProperties.h` | `ffi::kVTPixelTransferPropertyKey_DestinationICCProfile` |
| `kVTPixelTransferPropertyKey_DestinationPixelAspectRatio` | constant | `VTPixelTransferProperties.h` | `ffi::kVTPixelTransferPropertyKey_DestinationPixelAspectRatio` |
| `kVTPixelTransferPropertyKey_DestinationTransferFunction` | constant | `VTPixelTransferProperties.h` | `ffi::kVTPixelTransferPropertyKey_DestinationTransferFunction` |
| `kVTPixelTransferPropertyKey_DestinationYCbCrMatrix` | constant | `VTPixelTransferProperties.h` | `ffi::kVTPixelTransferPropertyKey_DestinationYCbCrMatrix` |
| `kVTPixelTransferPropertyKey_DownsamplingMode` | constant | `VTPixelTransferProperties.h` | `ffi::kVTPixelTransferPropertyKey_DownsamplingMode` |
| `kVTPixelTransferPropertyKey_RealTime` | constant | `VTPixelTransferProperties.h` | `ffi::kVTPixelTransferPropertyKey_RealTime` |
| `kVTPixelTransferPropertyKey_ScalingMode` | constant | `VTPixelTransferProperties.h` | `ffi::kVTPixelTransferPropertyKey_ScalingMode` |
| `kVTScalingMode_CropSourceToCleanAperture` | constant | `VTPixelTransferProperties.h` | `ffi::kVTScalingMode_CropSourceToCleanAperture` |
| `kVTScalingMode_Letterbox` | constant | `VTPixelTransferProperties.h` | `ffi::kVTScalingMode_Letterbox` |
| `kVTScalingMode_Normal` | constant | `VTPixelTransferProperties.h` | `ffi::kVTScalingMode_Normal` |
| `kVTScalingMode_Trim` | constant | `VTPixelTransferProperties.h` | `ffi::kVTScalingMode_Trim` |
| `VTPixelTransferSessionCreate` | function | `VTPixelTransferSession.h` | `ffi::VTPixelTransferSessionCreate` |
| `VTPixelTransferSessionGetTypeID` | function | `VTPixelTransferSession.h` | `ffi::VTPixelTransferSessionGetTypeID` |
| `VTPixelTransferSessionInvalidate` | function | `VTPixelTransferSession.h` | `ffi::VTPixelTransferSessionInvalidate` |
| `VTPixelTransferSessionRef` | opaque ref | `VTPixelTransferSession.h` | `ffi::VTPixelTransferSessionRef` |
| `VTPixelTransferSessionTransferImage` | function | `VTPixelTransferSession.h` | `ffi::VTPixelTransferSessionTransferImage` |
| `VTRegisterProfessionalVideoWorkflowVideoDecoders` | function | `VTProfessionalVideoWorkflow.h` | `ffi::VTRegisterProfessionalVideoWorkflowVideoDecoders` |
| `VTRegisterProfessionalVideoWorkflowVideoEncoders` | function | `VTProfessionalVideoWorkflow.h` | `ffi::VTRegisterProfessionalVideoWorkflowVideoEncoders` |
| `VTRAWProcessingSessionCompleteFrames` | function | `VTRAWProcessingSession.h` | `ffi::VTRAWProcessingSessionCompleteFrames` |
| `VTRAWProcessingSessionCopyProcessingParameters` | function | `VTRAWProcessingSession.h` | `ffi::VTRAWProcessingSessionCopyProcessingParameters` |
| `VTRAWProcessingSessionCreate` | function | `VTRAWProcessingSession.h` | `ffi::VTRAWProcessingSessionCreate` |
| `VTRAWProcessingSessionGetTypeID` | function | `VTRAWProcessingSession.h` | `ffi::VTRAWProcessingSessionGetTypeID` |
| `VTRAWProcessingSessionInvalidate` | function | `VTRAWProcessingSession.h` | `ffi::VTRAWProcessingSessionInvalidate` |
| `VTRAWProcessingSessionProcessFrame` | function | `VTRAWProcessingSession.h` | `RawProcessingSession::process via swift-bridge/Sources/VideoToolboxBridge/RAWProcessing.swift` |
| `VTRAWProcessingSessionRef` | opaque ref | `VTRAWProcessingSession.h` | `ffi::VTRAWProcessingSessionRef` |
| `VTRAWProcessingSessionSetProcessingParameters` | function | `VTRAWProcessingSession.h` | `ffi::VTRAWProcessingSessionSetProcessingParameters` |
| `kVTRAWProcessingParameterListElement_Description` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameterListElement_Description` |
| `kVTRAWProcessingParameterListElement_Label` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameterListElement_Label` |
| `kVTRAWProcessingParameterListElement_ListElementID` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameterListElement_ListElementID` |
| `kVTRAWProcessingParameterValueType_Boolean` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameterValueType_Boolean` |
| `kVTRAWProcessingParameterValueType_Float` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameterValueType_Float` |
| `kVTRAWProcessingParameterValueType_Integer` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameterValueType_Integer` |
| `kVTRAWProcessingParameterValueType_List` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameterValueType_List` |
| `kVTRAWProcessingParameterValueType_SubGroup` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameterValueType_SubGroup` |
| `kVTRAWProcessingParameter_CameraValue` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_CameraValue` |
| `kVTRAWProcessingParameter_CurrentValue` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_CurrentValue` |
| `kVTRAWProcessingParameter_Description` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_Description` |
| `kVTRAWProcessingParameter_Enabled` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_Enabled` |
| `kVTRAWProcessingParameter_InitialValue` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_InitialValue` |
| `kVTRAWProcessingParameter_Key` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_Key` |
| `kVTRAWProcessingParameter_ListArray` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_ListArray` |
| `kVTRAWProcessingParameter_MaximumValue` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_MaximumValue` |
| `kVTRAWProcessingParameter_MinimumValue` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_MinimumValue` |
| `kVTRAWProcessingParameter_Name` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_Name` |
| `kVTRAWProcessingParameter_NeutralValue` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_NeutralValue` |
| `kVTRAWProcessingParameter_SubGroup` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_SubGroup` |
| `kVTRAWProcessingParameter_ValueType` | constant | `VTRAWProcessingSession.h` | `ffi::kVTRAWProcessingParameter_ValueType` |
| `VTSessionCopyProperty` | function | `VTSession.h` | `ffi::VTSessionCopyProperty` |
| `VTSessionCopySerializableProperties` | function | `VTSession.h` | `ffi::VTSessionCopySerializableProperties` |
| `VTSessionCopySupportedPropertyDictionary` | function | `VTSession.h` | `ffi::VTSessionCopySupportedPropertyDictionary` |
| `VTSessionSetProperties` | function | `VTSession.h` | `ffi::VTSessionSetProperties` |
| `VTSessionSetProperty` | function | `VTSession.h` | `ffi::VTSessionSetProperty` |
| `VTCreateCGImageFromCVPixelBuffer` | function | `VTUtilities.h` | `ffi::VTCreateCGImageFromCVPixelBuffer` |
| `VTCopyVideoEncoderList` | function | `VTVideoEncoderList.h` | `ffi::VTCopyVideoEncoderList` |
| `kVTVideoEncoderList_CodecName` | constant | `VTVideoEncoderList.h` | `ffi::kVTVideoEncoderList_CodecName` |
| `kVTVideoEncoderList_CodecType` | constant | `VTVideoEncoderList.h` | `ffi::kVTVideoEncoderList_CodecType` |
| `kVTVideoEncoderList_DisplayName` | constant | `VTVideoEncoderList.h` | `ffi::kVTVideoEncoderList_DisplayName` |
| `kVTVideoEncoderList_EncoderID` | constant | `VTVideoEncoderList.h` | `ffi::kVTVideoEncoderList_EncoderID` |
| `kVTVideoEncoderList_EncoderName` | constant | `VTVideoEncoderList.h` | `ffi::kVTVideoEncoderList_EncoderName` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `kVTAlphaChannelMode_PremultipliedAlpha` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTAlphaChannelMode_StraightAlpha` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCameraCalibrationExtrinsicOriginSource_StereoCameraSystemBaseline` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCameraCalibrationLensAlgorithmKind_ParametricLens` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCameraCalibrationLensDomain_Color` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCameraCalibrationLensRole_Left` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCameraCalibrationLensRole_Mono` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCameraCalibrationLensRole_Right` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPreset_Balanced` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPreset_HighQuality` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPreset_HighSpeed` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPreset_VideoConferencing` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOrientationQuaternion` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOriginSource` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrix` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixProjectionOffset` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixReferenceDimensions` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_LensAlgorithmKind` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_LensDistortions` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_LensDomain` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialX` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialY` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_LensIdentifier` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_LensRole` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyCameraCalibrationKey_RadialAngleLimit` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_AllowOpenGOP` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_AllowTemporalCompression` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_AlphaChannelMode` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_AspectRatio16x9` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_BaseLayerBitRateFraction` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_BaseLayerFrameRate` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_BaseLayerFrameRateFraction` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_CalculateMeanSquaredError` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_CameraCalibrationDataLensCollection` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_CleanAperture` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_ConstantBitRate` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_ContentLightLevelInfo` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_DataRateLimits` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_Depth` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_EnableLTR` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_EncoderID` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_EstimatedAverageBytesPerFrame` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_ExpectedDuration` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_FieldCount` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_FieldDetail` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_GammaLevel` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_HDRMetadataInsertionMode` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_HasLeftStereoEyeView` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_HasRightStereoEyeView` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_HeroEye` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_HorizontalDisparityAdjustment` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_HorizontalFieldOfView` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MVHEVCLeftAndRightViewIDs` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MVHEVCVideoLayerIDs` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MVHEVCViewIDs` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MasteringDisplayColorVolume` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MaxAllowedFrameQP` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MaxFrameDelayCount` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MaxH264SliceBytes` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MaximizePowerEfficiency` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MaximumRealTimeFrameRate` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MinAllowedFrameQP` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MoreFramesAfterEnd` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_MoreFramesBeforeStart` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_NumberOfPendingFrames` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_OutputBitDepth` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_PixelAspectRatio` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_PixelBufferPoolIsShared` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_PixelTransferProperties` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_PreserveAlphaChannel` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_PreserveDynamicHDRMetadata` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_ProgressiveScan` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_ProjectionKind` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_RecommendedParallelizationLimit` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumDuration` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumFrameCount` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_ReferenceBufferCount` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_SourceFrameCount` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_SpatialAdaptiveQPLevel` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_StereoCameraBaseline` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_SuggestedLookAheadFrameCount` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_SupportedPresetDictionaries` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_SupportsBaseFrameQP` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_TargetQualityForAlpha` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_UsingGPURegistryID` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_UsingHardwareAcceleratedVideoEncoder` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_VBVBufferDuration` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_VBVInitialDelayPercentage` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_VBVMaxBitRate` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_VariableBitRate` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTCompressionPropertyKey_ViewPackingKind` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTEncodeFrameOptionKey_AcknowledgedLTRTokens` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTEncodeFrameOptionKey_BaseFrameQP` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTEncodeFrameOptionKey_ForceKeyFrame` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTEncodeFrameOptionKey_ForceLTRRefresh` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTHDRMetadataInsertionMode_Auto` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTHDRMetadataInsertionMode_None` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTHDRMetadataInsertionMode_RequestSDRRangePreservation` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTHeroEye_Left` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTHeroEye_Right` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTProjectionKind_Equirectangular` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTProjectionKind_HalfEquirectangular` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTProjectionKind_ParametricImmersive` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTProjectionKind_Rectilinear` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTSampleAttachmentKey_QualityMetrics` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTSampleAttachmentKey_RequireLTRAcknowledgementToken` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTSampleAttachmentQualityMetricsKey_ChromaBlueMeanSquaredError` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTSampleAttachmentQualityMetricsKey_ChromaRedMeanSquaredError` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTSampleAttachmentQualityMetricsKey_LumaMeanSquaredError` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTVideoEncoderSpecification_EnableLowLatencyRateControl` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTVideoEncoderSpecification_PreferredEncoderGPURegistryID` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTVideoEncoderSpecification_RequiredEncoderGPURegistryID` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTViewPackingKind_OverUnder` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `kVTViewPackingKind_SideBySide` | constant | `VTCompressionProperties.h` | Property key / profile constant is not exported in ffi; compression wrappers cover only a subset. |
| `VTCompressionSessionEncodeFrameWithOutputHandler` | function | `VTCompressionSession.h` | Block-based output-handler variant is not wrapped. |
| `VTCompressionSessionEncodeMultiImageFrame` | function | `VTCompressionSession.h` | Multi-image / stereo MV-HEVC encode path is not wrapped. |
| `VTCompressionSessionEncodeMultiImageFrameWithOutputHandler` | function | `VTCompressionSession.h` | Block-based output-handler variant is not wrapped. |
| `VTCompressionSessionOptionFlags` | options | `VTCompressionSession.h` | Typed flag alias is not exposed; current APIs use raw integers or fixed defaults. |
| `VTIsStereoMVHEVCEncodeSupported` | function | `VTCompressionSession.h` | No public Rust or ffi wrapper exposes this top-level framework symbol. |
| `kVTVideoEncoderSpecification_EncoderID` | constant | `VTCompressionSession.h` | CompressionSessionBuilder does not expose encoder-selection dictionaries. |
| `kVTDecodeFrameOptionKey_ContentAnalyzerCropRectangle` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecodeFrameOptionKey_ContentAnalyzerRotation` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_AllowBitstreamToChangeFrameDimensions` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_ContentHasInterframeDependencies` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_DecoderProducesRAWOutput` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_DeinterlaceMode` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_FieldMode` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_GeneratePerFrameHDRDisplayMetadata` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_MaximizePowerEfficiency` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_OnlyTheseFrames` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_PixelBufferPool` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_PixelBufferPoolIsShared` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_PixelTransferProperties` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_ReducedCoefficientDecode` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_ReducedFrameDelivery` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_ReducedResolutionDecode` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_RequestRAWOutput` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_RequestedMVHEVCVideoLayerIDs` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_SuggestedQualityOfServiceTiers` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByQuality` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_ThreadCount` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionPropertyKey_UsingGPURegistryID` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_DeinterlaceMode_Temporal` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_DeinterlaceMode_VerticalFilter` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_FieldMode_BothFields` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_FieldMode_BottomFieldOnly` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_FieldMode_DeinterlaceFields` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_FieldMode_SingleField` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_FieldMode_TopFieldOnly` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_OnlyTheseFrames_AllFrames` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_OnlyTheseFrames_IFrames` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_OnlyTheseFrames_KeyFrames` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_OnlyTheseFrames_NonDroppableFrames` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionProperty_TemporalLevelLimit` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionResolutionKey_Height` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTDecompressionResolutionKey_Width` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID` | constant | `VTDecompressionProperties.h` | Property key / decoder-specification constant is not exported in ffi. |
| `VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler` | function | `VTDecompressionSession.h` | No public Rust or ffi wrapper exposes this top-level framework symbol. |
| `VTDecompressionSessionDecodeFrameWithOptions` | function | `VTDecompressionSession.h` | Options-capable decode variants are not wrapped. |
| `VTDecompressionSessionDecodeFrameWithOptionsAndOutputHandler` | function | `VTDecompressionSession.h` | Options-capable decode variants are not wrapped. |
| `VTDecompressionSessionDecodeFrameWithOutputHandler` | function | `VTDecompressionSession.h` | Block-based output-handler variant is not wrapped. |
| `VTDecompressionSessionSetMultiImageCallback` | function | `VTDecompressionSession.h` | Stereo / multi-image decode callback API is not wrapped. |
| `VTIsStereoMVHEVCDecodeSupported` | function | `VTDecompressionSession.h` | No public Rust or ffi wrapper exposes this top-level framework symbol. |
| `VTDecodeFrameFlags` | options | `VTErrors.h` | Typed flag alias is not exposed; current APIs use raw integers or fixed defaults. |
| `VTDecodeInfoFlags` | options | `VTErrors.h` | Typed flag alias is not exposed; current APIs use raw integers or fixed defaults. |
| `VTFrameRateConversionConfigurationQualityPrioritization` | enum | `VTFrameProcessor_FrameRateConversion.h` | Frame-processor wrappers do not expose this configuration enum. |
| `VTFrameRateConversionConfigurationRevision` | enum | `VTFrameProcessor_FrameRateConversion.h` | Frame-processor wrappers do not expose this configuration enum. |
| `VTMotionBlurConfigurationQualityPrioritization` | enum | `VTFrameProcessor_MotionBlur.h` | Frame-processor wrappers do not expose this configuration enum. |
| `VTMotionBlurConfigurationRevision` | enum | `VTFrameProcessor_MotionBlur.h` | Frame-processor wrappers do not expose this configuration enum. |
| `VTOpticalFlowConfigurationQualityPrioritization` | enum | `VTFrameProcessor_OpticalFlow.h` | Frame-processor wrappers do not expose this configuration enum. |
| `VTOpticalFlowConfigurationRevision` | enum | `VTFrameProcessor_OpticalFlow.h` | Frame-processor wrappers do not expose this configuration enum. |
| `VTSuperResolutionScalerConfigurationInputType` | enum | `VTFrameProcessor_SuperResolutionScaler.h` | Frame-processor wrappers do not expose this configuration enum. |
| `VTSuperResolutionScalerConfigurationQualityPrioritization` | enum | `VTFrameProcessor_SuperResolutionScaler.h` | Frame-processor wrappers do not expose this configuration enum. |
| `VTSuperResolutionScalerConfigurationRevision` | enum | `VTFrameProcessor_SuperResolutionScaler.h` | Frame-processor wrappers do not expose this configuration enum. |
| `VTFrameSiloCallBlockForEachSampleBuffer` | function | `VTFrameSilo.h` | Crate wraps the function-callback enumerator, not the block-based variant. |
| `VTHDRPerFrameMetadataGenerationSessionGetTypeID` | function | `VTHDRPerFrameMetadataGenerationSession.h` | HdrMetadataSession hardcodes default options and does not expose HDR-format constants or the type ID. |
| `kVTHDRPerFrameMetadataGenerationHDRFormatType_DolbyVision` | constant | `VTHDRPerFrameMetadataGenerationSession.h` | HdrMetadataSession hardcodes default options and does not expose HDR-format constants or the type ID. |
| `kVTHDRPerFrameMetadataGenerationOptionsKey_HDRFormats` | constant | `VTHDRPerFrameMetadataGenerationSession.h` | HdrMetadataSession hardcodes default options and does not expose HDR-format constants or the type ID. |
| `VTMotionEstimationFrameFlags` | options | `VTMotionEstimationSession.h` | Typed flag alias is not exposed; current APIs use raw integers or fixed defaults. |
| `VTMotionEstimationInfoFlags` | options | `VTMotionEstimationSession.h` | Typed flag alias is not exposed; current APIs use raw integers or fixed defaults. |
| `kVTMotionEstimationSessionCreationOption_Label` | constant | `VTMotionEstimationSessionProperties.h` | MotionEstimationSession::new always passes null creation options; these option keys are not exposed. |
| `kVTMotionEstimationSessionCreationOption_MotionVectorSize` | constant | `VTMotionEstimationSessionProperties.h` | MotionEstimationSession::new always passes null creation options; these option keys are not exposed. |
| `kVTMotionEstimationSessionCreationOption_UseMultiPassSearch` | constant | `VTMotionEstimationSessionProperties.h` | MotionEstimationSession::new always passes null creation options; these option keys are not exposed. |
| `kVTRAWProcessingPropertyKey_MetadataForSidecarFile` | constant | `VTRAWProcessingProperties.h` | RawProcessingSession does not expose these RAW-session property keys. |
| `kVTRAWProcessingPropertyKey_MetalDeviceRegistryID` | constant | `VTRAWProcessingProperties.h` | RawProcessingSession does not expose these RAW-session property keys. |
| `kVTRAWProcessingPropertyKey_OutputColorAttachments` | constant | `VTRAWProcessingProperties.h` | RawProcessingSession does not expose these RAW-session property keys. |
| `VTRAWProcessingSessionSetParameterChangedHandler` | function | `VTRAWProcessingSession.h` | Parameter-change callback API is not exposed. |
| `kVTPropertyDocumentationKey` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertyReadWriteStatusKey` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertyReadWriteStatus_ReadOnly` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertyReadWriteStatus_ReadWrite` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertyShouldBeSerializedKey` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertySupportedValueListKey` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertySupportedValueMaximumKey` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertySupportedValueMinimumKey` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertyTypeKey` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertyType_Boolean` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertyType_Enumeration` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `kVTPropertyType_Number` | constant | `VTSession.h` | Generic VTSession property APIs are wrapped, but these metadata constants are not exported. |
| `VTCopyRAWProcessorExtensionProperties` | function | `VTUtilities.h` | utilities module omits supplemental decoder and extension-property helpers. |
| `VTCopyVideoDecoderExtensionProperties` | function | `VTUtilities.h` | utilities module omits supplemental decoder and extension-property helpers. |
| `VTRegisterSupplementalVideoDecoderIfAvailable` | function | `VTUtilities.h` | utilities module omits supplemental decoder and extension-property helpers. |
| `kVTExtensionProperties_CodecNameKey` | constant | `VTUtilities.h` | utilities module omits supplemental decoder and extension-property helpers. |
| `kVTExtensionProperties_ContainingBundleNameKey` | constant | `VTUtilities.h` | utilities module omits supplemental decoder and extension-property helpers. |
| `kVTExtensionProperties_ContainingBundleURLKey` | constant | `VTUtilities.h` | utilities module omits supplemental decoder and extension-property helpers. |
| `kVTExtensionProperties_ExtensionIdentifierKey` | constant | `VTUtilities.h` | utilities module omits supplemental decoder and extension-property helpers. |
| `kVTExtensionProperties_ExtensionNameKey` | constant | `VTUtilities.h` | utilities module omits supplemental decoder and extension-property helpers. |
| `kVTExtensionProperties_ExtensionURLKey` | constant | `VTUtilities.h` | utilities module omits supplemental decoder and extension-property helpers. |
| `VTCopySupportedPropertyDictionaryForEncoder` | function | `VTVideoEncoderList.h` | encoder_list exposes basic fields only; this extended encoder-list symbol is not surfaced. |
| `kVTVideoEncoderListOption_IncludeStandardDefinitionDVEncoders` | constant | `VTVideoEncoderList.h` | encoder_list exposes basic fields only; this extended encoder-list symbol is not surfaced. |
| `kVTVideoEncoderList_GPURegistryID` | constant | `VTVideoEncoderList.h` | encoder_list exposes basic fields only; this extended encoder-list symbol is not surfaced. |
| `kVTVideoEncoderList_InstanceLimit` | constant | `VTVideoEncoderList.h` | encoder_list exposes basic fields only; this extended encoder-list symbol is not surfaced. |
| `kVTVideoEncoderList_IsHardwareAccelerated` | constant | `VTVideoEncoderList.h` | encoder_list exposes basic fields only; this extended encoder-list symbol is not surfaced. |
| `kVTVideoEncoderList_PerformanceRating` | constant | `VTVideoEncoderList.h` | encoder_list exposes basic fields only; this extended encoder-list symbol is not surfaced. |
| `kVTVideoEncoderList_QualityRating` | constant | `VTVideoEncoderList.h` | encoder_list exposes basic fields only; this extended encoder-list symbol is not surfaced. |
| `kVTVideoEncoderList_SupportedSelectionProperties` | constant | `VTVideoEncoderList.h` | encoder_list exposes basic fields only; this extended encoder-list symbol is not surfaced. |
| `kVTVideoEncoderList_SupportsFrameReordering` | constant | `VTVideoEncoderList.h` | encoder_list exposes basic fields only; this extended encoder-list symbol is not surfaced. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `VTRAWProcessingSessionSetParameterChangedHander` | function | `VTRAWProcessingSession.h` | Deprecated misspelled alias; crate only targets the replacement spelling. | `API_DEPRECATED_WITH_REPLACEMENT("VTRAWProcessingSessionSetParameterChangedHandler", macos(15.0, 26.0))` |

