// VideoToolbox bridge — VTFrameProcessor capability queries.
//
// The processWithCommandBuffer pipeline lives in a future release;
// this file ships just the static `isSupported` checks so callers
// can detect capability at runtime without crashing on older
// systems where the classes aren't linkable.

import Foundation
import VideoToolbox

@_cdecl("vt_super_resolution_is_supported")
public func vt_super_resolution_is_supported() -> Bool {
    if #available(macOS 26.0, *) {
        return VTSuperResolutionScalerConfiguration.isSupported
    }
    return false
}

@_cdecl("vt_motion_blur_is_supported")
public func vt_motion_blur_is_supported() -> Bool {
    if #available(macOS 15.4, *) {
        return VTMotionBlurConfiguration.isSupported
    }
    return false
}

@_cdecl("vt_temporal_noise_filter_is_supported")
public func vt_temporal_noise_filter_is_supported() -> Bool {
    if #available(macOS 26.0, *) {
        return VTTemporalNoiseFilterConfiguration.isSupported
    }
    return false
}

@_cdecl("vt_frame_rate_conversion_is_supported")
public func vt_frame_rate_conversion_is_supported() -> Bool {
    if #available(macOS 15.4, *) {
        return VTFrameRateConversionConfiguration.isSupported
    }
    return false
}

@_cdecl("vt_low_latency_super_resolution_is_supported")
public func vt_low_latency_super_resolution_is_supported() -> Bool {
    if #available(macOS 26.0, *) {
        return VTLowLatencySuperResolutionScalerConfiguration.isSupported
    }
    return false
}

@_cdecl("vt_low_latency_frame_interpolation_is_supported")
public func vt_low_latency_frame_interpolation_is_supported() -> Bool {
    if #available(macOS 26.0, *) {
        return VTLowLatencyFrameInterpolationConfiguration.isSupported
    }
    return false
}

@_cdecl("vt_optical_flow_is_supported")
public func vt_optical_flow_is_supported() -> Bool {
    if #available(macOS 15.4, *) {
        return VTOpticalFlowConfiguration.isSupported
    }
    return false
}

/// Fill `out_buf` (length `max`) with VTSuperResolutionScalerConfiguration's
/// supportedScaleFactors. Returns the actual number of values written.
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
