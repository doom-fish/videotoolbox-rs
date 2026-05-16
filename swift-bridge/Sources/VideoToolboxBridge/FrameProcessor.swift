// VTFrameProcessor bridge — capability queries + full per-filter
// pipeline (start/process/end) for all 7 effects exposed by the
// framework on macOS 26.

import CoreVideo
import Foundation
import VideoToolbox

// MARK: - Capability queries

@_cdecl("vt_super_resolution_is_supported")
public func vt_super_resolution_is_supported() -> Bool {
    if #available(macOS 26.0, *) { return VTSuperResolutionScalerConfiguration.isSupported }
    return false
}

@_cdecl("vt_motion_blur_is_supported")
public func vt_motion_blur_is_supported() -> Bool {
    if #available(macOS 15.4, *) { return VTMotionBlurConfiguration.isSupported }
    return false
}

@_cdecl("vt_temporal_noise_filter_is_supported")
public func vt_temporal_noise_filter_is_supported() -> Bool {
    if #available(macOS 26.0, *) { return VTTemporalNoiseFilterConfiguration.isSupported }
    return false
}

@_cdecl("vt_frame_rate_conversion_is_supported")
public func vt_frame_rate_conversion_is_supported() -> Bool {
    if #available(macOS 15.4, *) { return VTFrameRateConversionConfiguration.isSupported }
    return false
}

@_cdecl("vt_low_latency_super_resolution_is_supported")
public func vt_low_latency_super_resolution_is_supported() -> Bool {
    if #available(macOS 26.0, *) { return VTLowLatencySuperResolutionScalerConfiguration.isSupported }
    return false
}

@_cdecl("vt_low_latency_frame_interpolation_is_supported")
public func vt_low_latency_frame_interpolation_is_supported() -> Bool {
    if #available(macOS 26.0, *) { return VTLowLatencyFrameInterpolationConfiguration.isSupported }
    return false
}

@_cdecl("vt_optical_flow_is_supported")
public func vt_optical_flow_is_supported() -> Bool {
    if #available(macOS 15.4, *) { return VTOpticalFlowConfiguration.isSupported }
    return false
}

/// Fill `out_buf` (capacity `max`) with VTSuperResolutionScalerConfiguration's
/// `supportedScaleFactors`. Returns the actual number of values written.
@_cdecl("vt_super_resolution_supported_scale_factors")
public func vt_super_resolution_supported_scale_factors(
    _ out_buf: UnsafeMutablePointer<UInt32>,
    _ max: Int
) -> Int {
    if #available(macOS 26.0, *) {
        let arr = VTSuperResolutionScalerConfiguration.supportedScaleFactors
        let n = min(arr.count, max)
        for i in 0..<n {
            out_buf[i] = UInt32(truncatingIfNeeded: arr[i])
        }
        return n
    }
    return 0
}

// MARK: - Pipeline: super-resolution scaler (macOS 26+)

/// Start a `VTFrameProcessor` session configured for super-resolution.
/// `inputIsImage` toggles between video (0) and image (1) input mode.
/// On success the retained `VTFrameProcessor` is returned via `out`.
@_cdecl("vt_super_resolution_start")
public func vt_super_resolution_start(
    _ frameWidth: Int,
    _ frameHeight: Int,
    _ scaleFactor: Int,
    _ usePrecomputedFlow: Bool,
    _ inputIsImage: Bool,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    if #available(macOS 26.0, *) {
        guard let cfg = VTSuperResolutionScalerConfiguration(
            frameWidth: frameWidth,
            frameHeight: frameHeight,
            scaleFactor: scaleFactor,
            inputType: inputIsImage ? .image : .video,
            usePrecomputedFlow: usePrecomputedFlow,
            qualityPrioritization: .normal,
            revision: .revision1
        ) else { return VTB_NOT_SUPPORTED }
        let p = VTFrameProcessor()
        do {
            try p.startSession(configuration: cfg)
            out.pointee = vtb_retain(p)
            return 0
        } catch { return vtb_status(from: error) }
    }
    return VTB_NOT_SUPPORTED
}

// MARK: - Pipeline: motion blur (macOS 15.4+)

@_cdecl("vt_motion_blur_start")
public func vt_motion_blur_start(
    _ frameWidth: Int,
    _ frameHeight: Int,
    _ usePrecomputedFlow: Bool,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    if #available(macOS 15.4, *) {
        guard let cfg = VTMotionBlurConfiguration(
            frameWidth: frameWidth,
            frameHeight: frameHeight,
            usePrecomputedFlow: usePrecomputedFlow,
            qualityPrioritization: .normal,
            revision: .revision1
        ) else { return VTB_NOT_SUPPORTED }
        let p = VTFrameProcessor()
        do {
            try p.startSession(configuration: cfg)
            out.pointee = vtb_retain(p)
            return 0
        } catch { return vtb_status(from: error) }
    }
    return VTB_NOT_SUPPORTED
}

// MARK: - Pipeline: temporal noise filter (macOS 26+)

@_cdecl("vt_temporal_noise_filter_start")
public func vt_temporal_noise_filter_start(
    _ frameWidth: Int,
    _ frameHeight: Int,
    _ sourcePixelFormat: UInt32,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    if #available(macOS 26.0, *) {
        guard let cfg = VTTemporalNoiseFilterConfiguration(
            frameWidth: frameWidth,
            frameHeight: frameHeight,
            sourcePixelFormat: sourcePixelFormat
        ) else { return VTB_NOT_SUPPORTED }
        let p = VTFrameProcessor()
        do {
            try p.startSession(configuration: cfg)
            out.pointee = vtb_retain(p)
            return 0
        } catch { return vtb_status(from: error) }
    }
    return VTB_NOT_SUPPORTED
}

// MARK: - Pipeline: frame-rate conversion (macOS 15.4+)

@_cdecl("vt_frame_rate_conversion_start")
public func vt_frame_rate_conversion_start(
    _ frameWidth: Int,
    _ frameHeight: Int,
    _ usePrecomputedFlow: Bool,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    if #available(macOS 15.4, *) {
        guard let cfg = VTFrameRateConversionConfiguration(
            frameWidth: frameWidth,
            frameHeight: frameHeight,
            usePrecomputedFlow: usePrecomputedFlow,
            qualityPrioritization: .normal,
            revision: .revision1
        ) else { return VTB_NOT_SUPPORTED }
        let p = VTFrameProcessor()
        do {
            try p.startSession(configuration: cfg)
            out.pointee = vtb_retain(p)
            return 0
        } catch { return vtb_status(from: error) }
    }
    return VTB_NOT_SUPPORTED
}

// MARK: - Pipeline: low-latency super-resolution scaler (macOS 26+)

@_cdecl("vt_low_latency_super_resolution_start")
public func vt_low_latency_super_resolution_start(
    _ frameWidth: Int,
    _ frameHeight: Int,
    _ scaleFactor: Float,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    if #available(macOS 26.0, *) {
        let cfg = VTLowLatencySuperResolutionScalerConfiguration(
            frameWidth: frameWidth,
            frameHeight: frameHeight,
            scaleFactor: scaleFactor
        )
        let p = VTFrameProcessor()
        do {
            try p.startSession(configuration: cfg)
            out.pointee = vtb_retain(p)
            return 0
        } catch { return vtb_status(from: error) }
    }
    return VTB_NOT_SUPPORTED
}

// MARK: - Pipeline: low-latency frame interpolation (macOS 26+)

@_cdecl("vt_low_latency_frame_interpolation_start")
public func vt_low_latency_frame_interpolation_start(
    _ frameWidth: Int,
    _ frameHeight: Int,
    _ numberOfInterpolatedFrames: Int,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    if #available(macOS 26.0, *) {
        guard let cfg = VTLowLatencyFrameInterpolationConfiguration(
            frameWidth: frameWidth,
            frameHeight: frameHeight,
            numberOfInterpolatedFrames: numberOfInterpolatedFrames
        ) else { return VTB_NOT_SUPPORTED }
        let p = VTFrameProcessor()
        do {
            try p.startSession(configuration: cfg)
            out.pointee = vtb_retain(p)
            return 0
        } catch { return vtb_status(from: error) }
    }
    return VTB_NOT_SUPPORTED
}

// MARK: - Pipeline: optical flow (macOS 15.4+)

@_cdecl("vt_optical_flow_start")
public func vt_optical_flow_start(
    _ frameWidth: Int,
    _ frameHeight: Int,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    if #available(macOS 15.4, *) {
        guard let cfg = VTOpticalFlowConfiguration(
            frameWidth: frameWidth,
            frameHeight: frameHeight,
            qualityPrioritization: .normal,
            revision: .revision1
        ) else { return VTB_NOT_SUPPORTED }
        let p = VTFrameProcessor()
        do {
            try p.startSession(configuration: cfg)
            out.pointee = vtb_retain(p)
            return 0
        } catch { return vtb_status(from: error) }
    }
    return VTB_NOT_SUPPORTED
}

// MARK: - Session lifecycle

@_cdecl("vt_frame_processor_end")
public func vt_frame_processor_end(_ processor: UnsafeMutableRawPointer) {
    if #available(macOS 15.4, *) {
        let p: VTFrameProcessor = vtb_borrow(processor)
        p.endSession()
    }
}

@_cdecl("vt_frame_processor_release")
public func vt_frame_processor_release(_ processor: UnsafeMutableRawPointer) {
    if #available(macOS 15.4, *) {
        vtb_release(processor, as: VTFrameProcessor.self)
    }
}
