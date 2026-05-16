//! `VTFrameProcessor` capability queries — `isSupported` + supported
//! scale factors via the new `frame_processor` Swift bridge.
//!
//! Apple's `VTFrameProcessor*Configuration` classes are
//! Objective-C-only — the pure-C FFI used by the rest of this crate
//! can't reach them. This module is feature-gated behind
//! `frame_processor` so the extra `swift build` step only happens
//! when callers opt in.

extern "C" {
    fn vt_super_resolution_is_supported() -> bool;
    fn vt_motion_blur_is_supported() -> bool;
    fn vt_temporal_noise_filter_is_supported() -> bool;
    fn vt_frame_rate_conversion_is_supported() -> bool;
    fn vt_low_latency_super_resolution_is_supported() -> bool;
    fn vt_low_latency_frame_interpolation_is_supported() -> bool;
    fn vt_optical_flow_is_supported() -> bool;
    fn vt_super_resolution_supported_scale_factors(out_buf: *mut u32, max: usize) -> usize;
}

/// Per-effect availability snapshot.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct FrameProcessorCapabilities {
    pub super_resolution: bool,
    pub motion_blur: bool,
    pub temporal_noise_filter: bool,
    pub frame_rate_conversion: bool,
    pub low_latency_super_resolution: bool,
    pub low_latency_frame_interpolation: bool,
    pub optical_flow: bool,
}

/// Query Apple's `VTFrameProcessor*Configuration.isSupported` for
/// each effect this binding wraps.
#[must_use]
pub fn frame_processor_capabilities() -> FrameProcessorCapabilities {
    unsafe {
        FrameProcessorCapabilities {
            super_resolution: vt_super_resolution_is_supported(),
            motion_blur: vt_motion_blur_is_supported(),
            temporal_noise_filter: vt_temporal_noise_filter_is_supported(),
            frame_rate_conversion: vt_frame_rate_conversion_is_supported(),
            low_latency_super_resolution: vt_low_latency_super_resolution_is_supported(),
            low_latency_frame_interpolation: vt_low_latency_frame_interpolation_is_supported(),
            optical_flow: vt_optical_flow_is_supported(),
        }
    }
}

/// Return the set of integer scale factors that
/// `VTSuperResolutionScalerConfiguration` accepts on this system
/// (e.g. `vec![2, 4]`). Empty if the API isn't available.
#[must_use]
pub fn super_resolution_supported_scale_factors() -> Vec<u32> {
    const MAX: usize = 16;
    let mut buf = vec![0u32; MAX];
    let n = unsafe { vt_super_resolution_supported_scale_factors(buf.as_mut_ptr(), MAX) };
    buf.truncate(n);
    buf
}
