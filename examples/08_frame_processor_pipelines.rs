//! Exercises the v0.10 motion-estimation + frame-processor surfaces:
//! creates a motion-estimation session, starts all 7 frame-processor
//! pipelines (skipping the ones the running OS doesn't support), and
//! verifies the FFI plumbing works end-to-end on an M-series Mac.

use videotoolbox::{
    frame_processor_capabilities, super_resolution_supported_scale_factors, FrameProcessor,
    MotionEstimationSession,
};

fn main() {
    let caps = frame_processor_capabilities();
    println!("Frame processor capabilities: {caps:?}");
    println!(
        "Super-res scale factors: {:?}",
        super_resolution_supported_scale_factors()
    );

    // Motion estimation (macOS 26+).
    match MotionEstimationSession::new(1920, 1080) {
        Ok(session) => {
            println!(
                "✅ MotionEstimationSession 1920x1080 created at {:p}",
                session.as_ptr()
            );
        }
        Err(e) => println!("⚠️  MotionEstimationSession unavailable: {e:?}"),
    }

    // Frame-processor pipelines.
    if caps.super_resolution {
        for scale in super_resolution_supported_scale_factors() {
            match FrameProcessor::start_super_resolution(1280, 720, scale as usize, false, false) {
                Ok(p) => println!("✅ SuperResolution {scale}x started at {:p}", p.as_ptr()),
                Err(e) => println!("⚠️  SuperResolution {scale}x failed: {e:?}"),
            }
        }
    }

    if caps.motion_blur {
        match FrameProcessor::start_motion_blur(1280, 720, false) {
            Ok(p) => println!("✅ MotionBlur started at {:p}", p.as_ptr()),
            Err(e) => println!("⚠️  MotionBlur failed: {e:?}"),
        }
    }

    if caps.temporal_noise_filter {
        // 'BGRA' as a packed FourCC
        let bgra = u32::from_be_bytes(*b"BGRA");
        match FrameProcessor::start_temporal_noise_filter(1280, 720, bgra) {
            Ok(p) => println!("✅ TemporalNoiseFilter started at {:p}", p.as_ptr()),
            Err(e) => println!("⚠️  TemporalNoiseFilter failed: {e:?}"),
        }
    }

    if caps.frame_rate_conversion {
        match FrameProcessor::start_frame_rate_conversion(1280, 720, false) {
            Ok(p) => println!("✅ FrameRateConversion started at {:p}", p.as_ptr()),
            Err(e) => println!("⚠️  FrameRateConversion failed: {e:?}"),
        }
    }

    if caps.low_latency_super_resolution {
        match FrameProcessor::start_low_latency_super_resolution(1280, 720, 2.0) {
            Ok(p) => println!(
                "✅ LowLatencySuperResolution 2x started at {:p}",
                p.as_ptr()
            ),
            Err(e) => println!("⚠️  LowLatencySuperResolution failed: {e:?}"),
        }
    }

    if caps.low_latency_frame_interpolation {
        match FrameProcessor::start_low_latency_frame_interpolation(1280, 720, 1) {
            Ok(p) => println!(
                "✅ LowLatencyFrameInterpolation started at {:p}",
                p.as_ptr()
            ),
            Err(e) => println!("⚠️  LowLatencyFrameInterpolation failed: {e:?}"),
        }
    }

    if caps.optical_flow {
        match FrameProcessor::start_optical_flow(1280, 720) {
            Ok(p) => println!("✅ OpticalFlow started at {:p}", p.as_ptr()),
            Err(e) => println!("⚠️  OpticalFlow failed: {e:?}"),
        }
    }
}
