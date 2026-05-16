use videotoolbox::{frame_processor_capabilities, super_resolution_supported_scale_factors};

fn main() {
    let caps = frame_processor_capabilities();
    println!("VTFrameProcessor capabilities on this system:");
    println!("  super_resolution               = {}", caps.super_resolution);
    println!("  motion_blur                    = {}", caps.motion_blur);
    println!("  temporal_noise_filter          = {}", caps.temporal_noise_filter);
    println!("  frame_rate_conversion          = {}", caps.frame_rate_conversion);
    println!("  low_latency_super_resolution   = {}", caps.low_latency_super_resolution);
    println!("  low_latency_frame_interpolation = {}", caps.low_latency_frame_interpolation);
    println!("  optical_flow                   = {}", caps.optical_flow);

    let scales = super_resolution_supported_scale_factors();
    println!("VTSuperResolutionScaler supported scale factors: {scales:?}");
}
