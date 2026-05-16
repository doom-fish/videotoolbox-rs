use videotoolbox::{
    frame_processor_capabilities, low_latency_super_resolution_supported_scale_factors,
    super_resolution_model_percentage_available, super_resolution_model_status,
    super_resolution_supported_scale_factors,
};

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;

    let caps = frame_processor_capabilities();
    println!("VTFrameProcessor capabilities on this system:");
    println!(
        "  super_resolution                = {}",
        caps.super_resolution
    );
    println!("  motion_blur                     = {}", caps.motion_blur);
    println!(
        "  temporal_noise_filter           = {}",
        caps.temporal_noise_filter
    );
    println!(
        "  frame_rate_conversion           = {}",
        caps.frame_rate_conversion
    );
    println!(
        "  low_latency_super_resolution    = {}",
        caps.low_latency_super_resolution
    );
    println!(
        "  low_latency_frame_interpolation = {}",
        caps.low_latency_frame_interpolation
    );
    println!("  optical_flow                    = {}", caps.optical_flow);

    let scales = super_resolution_supported_scale_factors();
    println!("VTSuperResolutionScaler supported scale factors: {scales:?}");
    if let Some(scale) = scales.first().copied() {
        println!(
            "VTSuperResolutionScaler model status at {WIDTH}x{HEIGHT}, scale {scale}: {:?} (progress {:?})",
            super_resolution_model_status(WIDTH, HEIGHT, scale as usize, false, true),
            super_resolution_model_percentage_available(WIDTH, HEIGHT, scale as usize, false, true),
        );
    }

    println!(
        "VTLowLatencySuperResolution supported scale factors at {WIDTH}x{HEIGHT}: {:?}",
        low_latency_super_resolution_supported_scale_factors(WIDTH, HEIGHT)
    );
}
