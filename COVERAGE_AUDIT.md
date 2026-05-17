# videotoolbox-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 449
VERIFIED: 448
GAPS: 0
EXEMPT: 1
COVERAGE_PCT: 100.00%

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
| `VTCompressionSessionEncodeFrameWithOutputHandler` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionEncodeFrameWithOutputHandler` |
| `VTCompressionSessionEncodeMultiImageFrameWithOutputHandler` | function | `VTCompressionSession.h` | `ffi::VTCompressionSessionEncodeMultiImageFrameWithOutputHandler` |
| `VTCompressionSessionEncodeMultiImageFrame` | function | `VTCompressionSession.h` | `CompressionSession::encode_multi_image + ffi::VTCompressionSessionEncodeMultiImageFrame` |
| `VTCompressionSessionOptionFlags` | options | `VTCompressionSession.h` | `ffi::VTCompressionSessionOptionFlags` |
| `VTCopySupportedPropertyDictionaryForEncoder` | function | `VTVideoEncoderList.h` | `encoder_list::supported_property_dictionary_for_encoder + ffi::VTCopySupportedPropertyDictionaryForEncoder` |
| `VTDecodeFrameFlags` | options | `VTErrors.h` | `ffi::VTDecodeFrameFlags` |
| `VTDecodeInfoFlags` | options | `VTErrors.h` | `ffi::VTDecodeInfoFlags` |
| `VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler` |
| `VTDecompressionSessionDecodeFrameWithOptionsAndOutputHandler` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionDecodeFrameWithOptionsAndOutputHandler` |
| `VTDecompressionSessionDecodeFrameWithOptions` | function | `VTDecompressionSession.h` | `DecompressionSession::decode_with_options + ffi::VTDecompressionSessionDecodeFrameWithOptions` |
| `VTDecompressionSessionDecodeFrameWithOutputHandler` | function | `VTDecompressionSession.h` | `ffi::VTDecompressionSessionDecodeFrameWithOutputHandler` |
| `VTDecompressionSessionSetMultiImageCallback` | function | `VTDecompressionSession.h` | `DecompressionSession::set_multi_image_callback + ffi::VTDecompressionSessionSetMultiImageCallback` |
| `VTIsStereoMVHEVCDecodeSupported` | function | `VTDecompressionSession.h` | `DecompressionSession::is_stereo_mvhevc_decode_supported + ffi::VTIsStereoMVHEVCDecodeSupported` |
| `VTIsStereoMVHEVCEncodeSupported` | function | `VTCompressionSession.h` | `CompressionSession::is_stereo_mvhevc_encode_supported + ffi::VTIsStereoMVHEVCEncodeSupported` |
| `kVTAlphaChannelMode_PremultipliedAlpha` | constant | `VTCompressionProperties.h` | `ffi::kVTAlphaChannelMode_PremultipliedAlpha` |
| `kVTAlphaChannelMode_StraightAlpha` | constant | `VTCompressionProperties.h` | `ffi::kVTAlphaChannelMode_StraightAlpha` |
| `kVTCameraCalibrationExtrinsicOriginSource_StereoCameraSystemBaseline` | constant | `VTCompressionProperties.h` | `ffi::kVTCameraCalibrationExtrinsicOriginSource_StereoCameraSystemBaseline` |
| `kVTCameraCalibrationLensAlgorithmKind_ParametricLens` | constant | `VTCompressionProperties.h` | `ffi::kVTCameraCalibrationLensAlgorithmKind_ParametricLens` |
| `kVTCameraCalibrationLensDomain_Color` | constant | `VTCompressionProperties.h` | `ffi::kVTCameraCalibrationLensDomain_Color` |
| `kVTCameraCalibrationLensRole_Left` | constant | `VTCompressionProperties.h` | `ffi::kVTCameraCalibrationLensRole_Left` |
| `kVTCameraCalibrationLensRole_Mono` | constant | `VTCompressionProperties.h` | `ffi::kVTCameraCalibrationLensRole_Mono` |
| `kVTCameraCalibrationLensRole_Right` | constant | `VTCompressionProperties.h` | `ffi::kVTCameraCalibrationLensRole_Right` |
| `kVTCompressionPreset_Balanced` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPreset_Balanced` |
| `kVTCompressionPreset_HighQuality` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPreset_HighQuality` |
| `kVTCompressionPreset_HighSpeed` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPreset_HighSpeed` |
| `kVTCompressionPreset_VideoConferencing` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPreset_VideoConferencing` |
| `kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOrientationQuaternion` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOrientationQuaternion` |
| `kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOriginSource` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOriginSource` |
| `kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixProjectionOffset` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixProjectionOffset` |
| `kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixReferenceDimensions` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixReferenceDimensions` |
| `kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrix` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrix` |
| `kVTCompressionPropertyCameraCalibrationKey_LensAlgorithmKind` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_LensAlgorithmKind` |
| `kVTCompressionPropertyCameraCalibrationKey_LensDistortions` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_LensDistortions` |
| `kVTCompressionPropertyCameraCalibrationKey_LensDomain` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_LensDomain` |
| `kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialX` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialX` |
| `kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialY` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialY` |
| `kVTCompressionPropertyCameraCalibrationKey_LensIdentifier` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_LensIdentifier` |
| `kVTCompressionPropertyCameraCalibrationKey_LensRole` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_LensRole` |
| `kVTCompressionPropertyCameraCalibrationKey_RadialAngleLimit` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyCameraCalibrationKey_RadialAngleLimit` |
| `kVTCompressionPropertyKey_AllowOpenGOP` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_AllowOpenGOP` |
| `kVTCompressionPropertyKey_AllowTemporalCompression` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_AllowTemporalCompression` |
| `kVTCompressionPropertyKey_AlphaChannelMode` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_AlphaChannelMode` |
| `kVTCompressionPropertyKey_AspectRatio16x9` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_AspectRatio16x9` |
| `kVTCompressionPropertyKey_BaseLayerBitRateFraction` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_BaseLayerBitRateFraction` |
| `kVTCompressionPropertyKey_BaseLayerFrameRateFraction` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_BaseLayerFrameRateFraction` |
| `kVTCompressionPropertyKey_BaseLayerFrameRate` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_BaseLayerFrameRate` |
| `kVTCompressionPropertyKey_CalculateMeanSquaredError` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_CalculateMeanSquaredError` |
| `kVTCompressionPropertyKey_CameraCalibrationDataLensCollection` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_CameraCalibrationDataLensCollection` |
| `kVTCompressionPropertyKey_CleanAperture` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_CleanAperture` |
| `kVTCompressionPropertyKey_ConstantBitRate` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ConstantBitRate` |
| `kVTCompressionPropertyKey_ContentLightLevelInfo` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ContentLightLevelInfo` |
| `kVTCompressionPropertyKey_DataRateLimits` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_DataRateLimits` |
| `kVTCompressionPropertyKey_Depth` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_Depth` |
| `kVTCompressionPropertyKey_EnableLTR` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_EnableLTR` |
| `kVTCompressionPropertyKey_EncoderID` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_EncoderID` |
| `kVTCompressionPropertyKey_EstimatedAverageBytesPerFrame` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_EstimatedAverageBytesPerFrame` |
| `kVTCompressionPropertyKey_ExpectedDuration` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ExpectedDuration` |
| `kVTCompressionPropertyKey_FieldCount` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_FieldCount` |
| `kVTCompressionPropertyKey_FieldDetail` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_FieldDetail` |
| `kVTCompressionPropertyKey_GammaLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_GammaLevel` |
| `kVTCompressionPropertyKey_HDRMetadataInsertionMode` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_HDRMetadataInsertionMode` |
| `kVTCompressionPropertyKey_HasLeftStereoEyeView` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_HasLeftStereoEyeView` |
| `kVTCompressionPropertyKey_HasRightStereoEyeView` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_HasRightStereoEyeView` |
| `kVTCompressionPropertyKey_HeroEye` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_HeroEye` |
| `kVTCompressionPropertyKey_HorizontalDisparityAdjustment` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_HorizontalDisparityAdjustment` |
| `kVTCompressionPropertyKey_HorizontalFieldOfView` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_HorizontalFieldOfView` |
| `kVTCompressionPropertyKey_MVHEVCLeftAndRightViewIDs` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MVHEVCLeftAndRightViewIDs` |
| `kVTCompressionPropertyKey_MVHEVCVideoLayerIDs` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MVHEVCVideoLayerIDs` |
| `kVTCompressionPropertyKey_MVHEVCViewIDs` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MVHEVCViewIDs` |
| `kVTCompressionPropertyKey_MasteringDisplayColorVolume` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MasteringDisplayColorVolume` |
| `kVTCompressionPropertyKey_MaxAllowedFrameQP` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MaxAllowedFrameQP` |
| `kVTCompressionPropertyKey_MaxFrameDelayCount` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MaxFrameDelayCount` |
| `kVTCompressionPropertyKey_MaxH264SliceBytes` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MaxH264SliceBytes` |
| `kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration` |
| `kVTCompressionPropertyKey_MaximizePowerEfficiency` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MaximizePowerEfficiency` |
| `kVTCompressionPropertyKey_MaximumRealTimeFrameRate` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MaximumRealTimeFrameRate` |
| `kVTCompressionPropertyKey_MinAllowedFrameQP` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MinAllowedFrameQP` |
| `kVTCompressionPropertyKey_MoreFramesAfterEnd` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MoreFramesAfterEnd` |
| `kVTCompressionPropertyKey_MoreFramesBeforeStart` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_MoreFramesBeforeStart` |
| `kVTCompressionPropertyKey_NumberOfPendingFrames` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_NumberOfPendingFrames` |
| `kVTCompressionPropertyKey_OutputBitDepth` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_OutputBitDepth` |
| `kVTCompressionPropertyKey_PixelAspectRatio` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_PixelAspectRatio` |
| `kVTCompressionPropertyKey_PixelBufferPoolIsShared` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_PixelBufferPoolIsShared` |
| `kVTCompressionPropertyKey_PixelTransferProperties` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_PixelTransferProperties` |
| `kVTCompressionPropertyKey_PreserveAlphaChannel` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_PreserveAlphaChannel` |
| `kVTCompressionPropertyKey_PreserveDynamicHDRMetadata` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_PreserveDynamicHDRMetadata` |
| `kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality` |
| `kVTCompressionPropertyKey_ProgressiveScan` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ProgressiveScan` |
| `kVTCompressionPropertyKey_ProjectionKind` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ProjectionKind` |
| `kVTCompressionPropertyKey_RecommendedParallelizationLimit` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_RecommendedParallelizationLimit` |
| `kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumDuration` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumDuration` |
| `kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumFrameCount` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumFrameCount` |
| `kVTCompressionPropertyKey_ReferenceBufferCount` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ReferenceBufferCount` |
| `kVTCompressionPropertyKey_SourceFrameCount` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_SourceFrameCount` |
| `kVTCompressionPropertyKey_SpatialAdaptiveQPLevel` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_SpatialAdaptiveQPLevel` |
| `kVTCompressionPropertyKey_StereoCameraBaseline` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_StereoCameraBaseline` |
| `kVTCompressionPropertyKey_SuggestedLookAheadFrameCount` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_SuggestedLookAheadFrameCount` |
| `kVTCompressionPropertyKey_SupportedPresetDictionaries` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_SupportedPresetDictionaries` |
| `kVTCompressionPropertyKey_SupportsBaseFrameQP` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_SupportsBaseFrameQP` |
| `kVTCompressionPropertyKey_TargetQualityForAlpha` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_TargetQualityForAlpha` |
| `kVTCompressionPropertyKey_UsingGPURegistryID` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_UsingGPURegistryID` |
| `kVTCompressionPropertyKey_UsingHardwareAcceleratedVideoEncoder` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_UsingHardwareAcceleratedVideoEncoder` |
| `kVTCompressionPropertyKey_VBVBufferDuration` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_VBVBufferDuration` |
| `kVTCompressionPropertyKey_VBVInitialDelayPercentage` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_VBVInitialDelayPercentage` |
| `kVTCompressionPropertyKey_VBVMaxBitRate` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_VBVMaxBitRate` |
| `kVTCompressionPropertyKey_VariableBitRate` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_VariableBitRate` |
| `kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes` |
| `kVTCompressionPropertyKey_ViewPackingKind` | constant | `VTCompressionProperties.h` | `ffi::kVTCompressionPropertyKey_ViewPackingKind` |
| `kVTDecodeFrameOptionKey_ContentAnalyzerCropRectangle` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecodeFrameOptionKey_ContentAnalyzerCropRectangle` |
| `kVTDecodeFrameOptionKey_ContentAnalyzerRotation` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecodeFrameOptionKey_ContentAnalyzerRotation` |
| `kVTDecompressionPropertyKey_AllowBitstreamToChangeFrameDimensions` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_AllowBitstreamToChangeFrameDimensions` |
| `kVTDecompressionPropertyKey_ContentHasInterframeDependencies` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_ContentHasInterframeDependencies` |
| `kVTDecompressionPropertyKey_DecoderProducesRAWOutput` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_DecoderProducesRAWOutput` |
| `kVTDecompressionPropertyKey_DeinterlaceMode` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_DeinterlaceMode` |
| `kVTDecompressionPropertyKey_FieldMode` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_FieldMode` |
| `kVTDecompressionPropertyKey_GeneratePerFrameHDRDisplayMetadata` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_GeneratePerFrameHDRDisplayMetadata` |
| `kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded` |
| `kVTDecompressionPropertyKey_MaximizePowerEfficiency` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_MaximizePowerEfficiency` |
| `kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded` |
| `kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded` |
| `kVTDecompressionPropertyKey_OnlyTheseFrames` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_OnlyTheseFrames` |
| `kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount` |
| `kVTDecompressionPropertyKey_PixelBufferPoolIsShared` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_PixelBufferPoolIsShared` |
| `kVTDecompressionPropertyKey_PixelBufferPool` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_PixelBufferPool` |
| `kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport` |
| `kVTDecompressionPropertyKey_PixelTransferProperties` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_PixelTransferProperties` |
| `kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata` |
| `kVTDecompressionPropertyKey_ReducedCoefficientDecode` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_ReducedCoefficientDecode` |
| `kVTDecompressionPropertyKey_ReducedFrameDelivery` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_ReducedFrameDelivery` |
| `kVTDecompressionPropertyKey_ReducedResolutionDecode` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_ReducedResolutionDecode` |
| `kVTDecompressionPropertyKey_RequestRAWOutput` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_RequestRAWOutput` |
| `kVTDecompressionPropertyKey_RequestedMVHEVCVideoLayerIDs` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_RequestedMVHEVCVideoLayerIDs` |
| `kVTDecompressionPropertyKey_SuggestedQualityOfServiceTiers` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_SuggestedQualityOfServiceTiers` |
| `kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance` |
| `kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByQuality` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByQuality` |
| `kVTDecompressionPropertyKey_ThreadCount` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_ThreadCount` |
| `kVTDecompressionPropertyKey_UsingGPURegistryID` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionPropertyKey_UsingGPURegistryID` |
| `kVTDecompressionProperty_DeinterlaceMode_Temporal` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_DeinterlaceMode_Temporal` |
| `kVTDecompressionProperty_DeinterlaceMode_VerticalFilter` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_DeinterlaceMode_VerticalFilter` |
| `kVTDecompressionProperty_FieldMode_BothFields` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_FieldMode_BothFields` |
| `kVTDecompressionProperty_FieldMode_BottomFieldOnly` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_FieldMode_BottomFieldOnly` |
| `kVTDecompressionProperty_FieldMode_DeinterlaceFields` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_FieldMode_DeinterlaceFields` |
| `kVTDecompressionProperty_FieldMode_SingleField` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_FieldMode_SingleField` |
| `kVTDecompressionProperty_FieldMode_TopFieldOnly` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_FieldMode_TopFieldOnly` |
| `kVTDecompressionProperty_OnlyTheseFrames_AllFrames` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_OnlyTheseFrames_AllFrames` |
| `kVTDecompressionProperty_OnlyTheseFrames_IFrames` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_OnlyTheseFrames_IFrames` |
| `kVTDecompressionProperty_OnlyTheseFrames_KeyFrames` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_OnlyTheseFrames_KeyFrames` |
| `kVTDecompressionProperty_OnlyTheseFrames_NonDroppableFrames` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_OnlyTheseFrames_NonDroppableFrames` |
| `kVTDecompressionProperty_TemporalLevelLimit` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionProperty_TemporalLevelLimit` |
| `kVTDecompressionResolutionKey_Height` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionResolutionKey_Height` |
| `kVTDecompressionResolutionKey_Width` | constant | `VTDecompressionProperties.h` | `ffi::kVTDecompressionResolutionKey_Width` |
| `kVTEncodeFrameOptionKey_AcknowledgedLTRTokens` | constant | `VTCompressionProperties.h` | `ffi::kVTEncodeFrameOptionKey_AcknowledgedLTRTokens` |
| `kVTEncodeFrameOptionKey_BaseFrameQP` | constant | `VTCompressionProperties.h` | `ffi::kVTEncodeFrameOptionKey_BaseFrameQP` |
| `kVTEncodeFrameOptionKey_ForceKeyFrame` | constant | `VTCompressionProperties.h` | `ffi::kVTEncodeFrameOptionKey_ForceKeyFrame` |
| `kVTEncodeFrameOptionKey_ForceLTRRefresh` | constant | `VTCompressionProperties.h` | `ffi::kVTEncodeFrameOptionKey_ForceLTRRefresh` |
| `kVTHDRMetadataInsertionMode_Auto` | constant | `VTCompressionProperties.h` | `ffi::kVTHDRMetadataInsertionMode_Auto` |
| `kVTHDRMetadataInsertionMode_None` | constant | `VTCompressionProperties.h` | `ffi::kVTHDRMetadataInsertionMode_None` |
| `kVTHDRMetadataInsertionMode_RequestSDRRangePreservation` | constant | `VTCompressionProperties.h` | `ffi::kVTHDRMetadataInsertionMode_RequestSDRRangePreservation` |
| `kVTHeroEye_Left` | constant | `VTCompressionProperties.h` | `ffi::kVTHeroEye_Left` |
| `kVTHeroEye_Right` | constant | `VTCompressionProperties.h` | `ffi::kVTHeroEye_Right` |
| `kVTProjectionKind_Equirectangular` | constant | `VTCompressionProperties.h` | `ffi::kVTProjectionKind_Equirectangular` |
| `kVTProjectionKind_HalfEquirectangular` | constant | `VTCompressionProperties.h` | `ffi::kVTProjectionKind_HalfEquirectangular` |
| `kVTProjectionKind_ParametricImmersive` | constant | `VTCompressionProperties.h` | `ffi::kVTProjectionKind_ParametricImmersive` |
| `kVTProjectionKind_Rectilinear` | constant | `VTCompressionProperties.h` | `ffi::kVTProjectionKind_Rectilinear` |
| `kVTSampleAttachmentKey_QualityMetrics` | constant | `VTCompressionProperties.h` | `ffi::kVTSampleAttachmentKey_QualityMetrics` |
| `kVTSampleAttachmentKey_RequireLTRAcknowledgementToken` | constant | `VTCompressionProperties.h` | `ffi::kVTSampleAttachmentKey_RequireLTRAcknowledgementToken` |
| `kVTSampleAttachmentQualityMetricsKey_ChromaBlueMeanSquaredError` | constant | `VTCompressionProperties.h` | `ffi::kVTSampleAttachmentQualityMetricsKey_ChromaBlueMeanSquaredError` |
| `kVTSampleAttachmentQualityMetricsKey_ChromaRedMeanSquaredError` | constant | `VTCompressionProperties.h` | `ffi::kVTSampleAttachmentQualityMetricsKey_ChromaRedMeanSquaredError` |
| `kVTSampleAttachmentQualityMetricsKey_LumaMeanSquaredError` | constant | `VTCompressionProperties.h` | `ffi::kVTSampleAttachmentQualityMetricsKey_LumaMeanSquaredError` |
| `kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder` | constant | `VTDecompressionProperties.h` | `ffi::kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder` |
| `kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID` | constant | `VTDecompressionProperties.h` | `ffi::kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID` |
| `kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder` | constant | `VTDecompressionProperties.h` | `ffi::kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder` |
| `kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID` | constant | `VTDecompressionProperties.h` | `ffi::kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID` |
| `kVTVideoEncoderListOption_IncludeStandardDefinitionDVEncoders` | constant | `VTVideoEncoderList.h` | `encoder_list::VideoEncoderListOptions + ffi::kVTVideoEncoderListOption_IncludeStandardDefinitionDVEncoders` |
| `kVTVideoEncoderList_GPURegistryID` | constant | `VTVideoEncoderList.h` | `encoder_list::available_video_encoder_details[_with_options] + ffi::kVTVideoEncoderList_GPURegistryID` |
| `kVTVideoEncoderList_InstanceLimit` | constant | `VTVideoEncoderList.h` | `encoder_list::available_video_encoder_details[_with_options] + ffi::kVTVideoEncoderList_InstanceLimit` |
| `kVTVideoEncoderList_IsHardwareAccelerated` | constant | `VTVideoEncoderList.h` | `encoder_list::available_video_encoder_details[_with_options] + ffi::kVTVideoEncoderList_IsHardwareAccelerated` |
| `kVTVideoEncoderList_PerformanceRating` | constant | `VTVideoEncoderList.h` | `encoder_list::available_video_encoder_details[_with_options] + ffi::kVTVideoEncoderList_PerformanceRating` |
| `kVTVideoEncoderList_QualityRating` | constant | `VTVideoEncoderList.h` | `encoder_list::available_video_encoder_details[_with_options] + ffi::kVTVideoEncoderList_QualityRating` |
| `kVTVideoEncoderList_SupportedSelectionProperties` | constant | `VTVideoEncoderList.h` | `encoder_list::available_video_encoder_details[_with_options] + ffi::kVTVideoEncoderList_SupportedSelectionProperties` |
| `kVTVideoEncoderList_SupportsFrameReordering` | constant | `VTVideoEncoderList.h` | `encoder_list::available_video_encoder_details[_with_options] + ffi::kVTVideoEncoderList_SupportsFrameReordering` |
| `kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder` | constant | `VTCompressionProperties.h` | `ffi::kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder` |
| `kVTVideoEncoderSpecification_EnableLowLatencyRateControl` | constant | `VTCompressionProperties.h` | `ffi::kVTVideoEncoderSpecification_EnableLowLatencyRateControl` |
| `kVTVideoEncoderSpecification_EncoderID` | constant | `VTCompressionSession.h` | `encoder_list::supported_property_dictionary_for_encoder + ffi::kVTVideoEncoderSpecification_EncoderID` |
| `kVTVideoEncoderSpecification_PreferredEncoderGPURegistryID` | constant | `VTCompressionProperties.h` | `ffi::kVTVideoEncoderSpecification_PreferredEncoderGPURegistryID` |
| `kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder` | constant | `VTCompressionProperties.h` | `ffi::kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder` |
| `kVTVideoEncoderSpecification_RequiredEncoderGPURegistryID` | constant | `VTCompressionProperties.h` | `ffi::kVTVideoEncoderSpecification_RequiredEncoderGPURegistryID` |
| `kVTViewPackingKind_OverUnder` | constant | `VTCompressionProperties.h` | `ffi::kVTViewPackingKind_OverUnder` |
| `kVTViewPackingKind_SideBySide` | constant | `VTCompressionProperties.h` | `ffi::kVTViewPackingKind_SideBySide` |

## 🟢 VERIFIED (v0.11.2 additions)
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `VTFrameRateConversionConfigurationQualityPrioritization` | enum | `VTFrameProcessor_FrameRateConversion.h` | `frame_processor::VTFrameRateConversionConfigurationQualityPrioritization` |
| `VTFrameRateConversionConfigurationRevision` | enum | `VTFrameProcessor_FrameRateConversion.h` | `frame_processor::VTFrameRateConversionConfigurationRevision` |
| `VTMotionBlurConfigurationQualityPrioritization` | enum | `VTFrameProcessor_MotionBlur.h` | `frame_processor::VTMotionBlurConfigurationQualityPrioritization` |
| `VTMotionBlurConfigurationRevision` | enum | `VTFrameProcessor_MotionBlur.h` | `frame_processor::VTMotionBlurConfigurationRevision` |
| `VTOpticalFlowConfigurationQualityPrioritization` | enum | `VTFrameProcessor_OpticalFlow.h` | `frame_processor::VTOpticalFlowConfigurationQualityPrioritization` |
| `VTOpticalFlowConfigurationRevision` | enum | `VTFrameProcessor_OpticalFlow.h` | `frame_processor::VTOpticalFlowConfigurationRevision` |
| `VTSuperResolutionScalerConfigurationInputType` | enum | `VTFrameProcessor_SuperResolutionScaler.h` | `frame_processor::VTSuperResolutionScalerConfigurationInputType` |
| `VTSuperResolutionScalerConfigurationQualityPrioritization` | enum | `VTFrameProcessor_SuperResolutionScaler.h` | `frame_processor::VTSuperResolutionScalerConfigurationQualityPrioritization` |
| `VTSuperResolutionScalerConfigurationRevision` | enum | `VTFrameProcessor_SuperResolutionScaler.h` | `frame_processor::VTSuperResolutionScalerConfigurationRevision` |
| `VTFrameSiloCallBlockForEachSampleBuffer` | function | `VTFrameSilo.h` | `ffi::VTFrameSiloCallBlockForEachSampleBuffer` |
| `VTHDRPerFrameMetadataGenerationSessionGetTypeID` | function | `VTHDRPerFrameMetadataGenerationSession.h` | `HdrMetadataSession::type_id` |
| `kVTHDRPerFrameMetadataGenerationHDRFormatType_DolbyVision` | constant | `VTHDRPerFrameMetadataGenerationSession.h` | `hdr_metadata::HdrMetadataFormat::DolbyVision + ffi::kVTHDRPerFrameMetadataGenerationHDRFormatType_DolbyVision` |
| `kVTHDRPerFrameMetadataGenerationOptionsKey_HDRFormats` | constant | `VTHDRPerFrameMetadataGenerationSession.h` | `HdrMetadataSession::new_with_formats + ffi::kVTHDRPerFrameMetadataGenerationOptionsKey_HDRFormats` |
| `VTMotionEstimationFrameFlags` | options | `VTMotionEstimationSession.h` | `ffi::VTMotionEstimationFrameFlags + MotionEstimationSession::estimate_with_options` |
| `VTMotionEstimationInfoFlags` | options | `VTMotionEstimationSession.h` | `ffi::VTMotionEstimationInfoFlags + MotionEstimationResult::info_flags` |
| `kVTMotionEstimationSessionCreationOption_Label` | constant | `VTMotionEstimationSessionProperties.h` | `MotionEstimationSessionOptions::label + ffi::kVTMotionEstimationSessionCreationOption_Label` |
| `kVTMotionEstimationSessionCreationOption_MotionVectorSize` | constant | `VTMotionEstimationSessionProperties.h` | `MotionEstimationSessionOptions::motion_vector_size + ffi::kVTMotionEstimationSessionCreationOption_MotionVectorSize` |
| `kVTMotionEstimationSessionCreationOption_UseMultiPassSearch` | constant | `VTMotionEstimationSessionProperties.h` | `MotionEstimationSessionOptions::use_multi_pass_search + ffi::kVTMotionEstimationSessionCreationOption_UseMultiPassSearch` |
| `kVTRAWProcessingPropertyKey_MetadataForSidecarFile` | constant | `VTRAWProcessingProperties.h` | `RawProcessingSession::metadata_for_sidecar_file + ffi::kVTRAWProcessingPropertyKey_MetadataForSidecarFile` |
| `kVTRAWProcessingPropertyKey_MetalDeviceRegistryID` | constant | `VTRAWProcessingProperties.h` | `RawProcessingSession::metal_device_registry_id + ffi::kVTRAWProcessingPropertyKey_MetalDeviceRegistryID` |
| `kVTRAWProcessingPropertyKey_OutputColorAttachments` | constant | `VTRAWProcessingProperties.h` | `RawProcessingSession::output_color_attachments + ffi::kVTRAWProcessingPropertyKey_OutputColorAttachments` |
| `VTRAWProcessingSessionSetParameterChangedHandler` | function | `VTRAWProcessingSession.h` | `RawProcessingSession::set_parameter_changed_handler / clear_parameter_changed_handler via swift-bridge` |
| `kVTPropertyDocumentationKey` | constant | `VTSession.h` | `ffi::kVTPropertyDocumentationKey` |
| `kVTPropertyReadWriteStatusKey` | constant | `VTSession.h` | `ffi::kVTPropertyReadWriteStatusKey` |
| `kVTPropertyReadWriteStatus_ReadOnly` | constant | `VTSession.h` | `ffi::kVTPropertyReadWriteStatus_ReadOnly` |
| `kVTPropertyReadWriteStatus_ReadWrite` | constant | `VTSession.h` | `ffi::kVTPropertyReadWriteStatus_ReadWrite` |
| `kVTPropertyShouldBeSerializedKey` | constant | `VTSession.h` | `ffi::kVTPropertyShouldBeSerializedKey` |
| `kVTPropertySupportedValueListKey` | constant | `VTSession.h` | `ffi::kVTPropertySupportedValueListKey` |
| `kVTPropertySupportedValueMaximumKey` | constant | `VTSession.h` | `ffi::kVTPropertySupportedValueMaximumKey` |
| `kVTPropertySupportedValueMinimumKey` | constant | `VTSession.h` | `ffi::kVTPropertySupportedValueMinimumKey` |
| `kVTPropertyTypeKey` | constant | `VTSession.h` | `ffi::kVTPropertyTypeKey` |
| `kVTPropertyType_Boolean` | constant | `VTSession.h` | `ffi::kVTPropertyType_Boolean` |
| `kVTPropertyType_Enumeration` | constant | `VTSession.h` | `ffi::kVTPropertyType_Enumeration` |
| `kVTPropertyType_Number` | constant | `VTSession.h` | `ffi::kVTPropertyType_Number` |
| `VTCopyRAWProcessorExtensionProperties` | function | `VTUtilities.h` | `utilities::copy_raw_processor_extension_properties + ffi::VTCopyRAWProcessorExtensionProperties` |
| `VTCopyVideoDecoderExtensionProperties` | function | `VTUtilities.h` | `utilities::copy_video_decoder_extension_properties + ffi::VTCopyVideoDecoderExtensionProperties` |
| `VTRegisterSupplementalVideoDecoderIfAvailable` | function | `VTUtilities.h` | `utilities::register_supplemental_video_decoder_if_available + ffi::VTRegisterSupplementalVideoDecoderIfAvailable` |
| `kVTExtensionProperties_CodecNameKey` | constant | `VTUtilities.h` | `ffi::kVTExtensionProperties_CodecNameKey` |
| `kVTExtensionProperties_ContainingBundleNameKey` | constant | `VTUtilities.h` | `ffi::kVTExtensionProperties_ContainingBundleNameKey` |
| `kVTExtensionProperties_ContainingBundleURLKey` | constant | `VTUtilities.h` | `ffi::kVTExtensionProperties_ContainingBundleURLKey` |
| `kVTExtensionProperties_ExtensionIdentifierKey` | constant | `VTUtilities.h` | `ffi::kVTExtensionProperties_ExtensionIdentifierKey` |
| `kVTExtensionProperties_ExtensionNameKey` | constant | `VTUtilities.h` | `ffi::kVTExtensionProperties_ExtensionNameKey` |
| `kVTExtensionProperties_ExtensionURLKey` | constant | `VTUtilities.h` | `ffi::kVTExtensionProperties_ExtensionURLKey` |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `VTRAWProcessingSessionSetParameterChangedHander` | function | `VTRAWProcessingSession.h` | Deprecated misspelled alias; crate only targets the replacement spelling. | `API_DEPRECATED_WITH_REPLACEMENT("VTRAWProcessingSessionSetParameterChangedHandler", macos(15.0, 26.0))` |

