#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::too_many_lines
)]

//! Run real `VTFrameProcessor` submissions on supported pipelines.
//!
//! This example allocates IOSurface-backed `CVPixelBuffer`s, wraps them as
//! `FrameProcessorFrame`s, and demonstrates both synchronous processing and
//! command-buffer integration.

use apple_cf::{
    cm::CMTime,
    cv::CVPixelBuffer,
    iosurface::{IOSurface, PlaneProperties},
};
use apple_metal::MetalDevice;
use videotoolbox::{
    frame_processor_capabilities, low_latency_super_resolution_supported_scale_factors,
    super_resolution_model_percentage_available, super_resolution_model_status,
    super_resolution_supported_scale_factors, FrameProcessor, FrameProcessorFrame,
    FrameProcessorOpticalFlow, FrameProcessorSubmissionMode, FrameRateConversionSubmissionMode,
    SuperResolutionModelStatus,
};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const TIMESCALE: i32 = 60;

const fn fcc(s: [u8; 4]) -> u32 {
    u32::from_be_bytes(s)
}

fn make_rgha_pixel_buffer(width: usize, height: usize) -> Result<CVPixelBuffer, String> {
    let bytes_per_element = 8;
    let bytes_per_row = width * bytes_per_element;
    let alloc_size = bytes_per_row * height;
    let surface = IOSurface::create_with_properties(
        width,
        height,
        fcc(*b"RGhA"),
        bytes_per_element,
        bytes_per_row,
        alloc_size,
        None,
    )
    .ok_or_else(|| "failed to allocate RGhA IOSurface".to_string())?;
    CVPixelBuffer::create_with_io_surface(&surface)
        .map_err(|status| format!("CVPixelBuffer::create_with_io_surface(RGhA): {status}"))
}

fn make_420v_pixel_buffer(width: usize, height: usize) -> Result<CVPixelBuffer, String> {
    let plane0_bpr = (width + 15) & !15;
    let plane1_bpr = (width + 15) & !15;
    let plane0_size = plane0_bpr * height;
    let plane1_size = plane1_bpr * (height / 2);
    let planes = [
        PlaneProperties {
            width,
            height,
            bytes_per_row: plane0_bpr,
            bytes_per_element: 1,
            offset: 0,
            size: plane0_size,
        },
        PlaneProperties {
            width: width / 2,
            height: height / 2,
            bytes_per_row: plane1_bpr,
            bytes_per_element: 2,
            offset: plane0_size,
            size: plane1_size,
        },
    ];
    let surface = IOSurface::create_with_properties(
        width,
        height,
        fcc(*b"420v"),
        1,
        plane0_bpr,
        plane0_size + plane1_size,
        Some(&planes),
    )
    .ok_or_else(|| "failed to allocate 420v IOSurface".to_string())?;
    CVPixelBuffer::create_with_io_surface(&surface)
        .map_err(|status| format!("CVPixelBuffer::create_with_io_surface(420v): {status}"))
}

fn make_frame(
    buffer: &CVPixelBuffer,
    frame_index: i64,
) -> Result<FrameProcessorFrame, videotoolbox::VTError> {
    FrameProcessorFrame::new(buffer, CMTime::new(frame_index, TIMESCALE))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let caps = frame_processor_capabilities();
    println!("Frame processor capabilities: {caps:?}");

    let frame0 = make_frame(&make_rgha_pixel_buffer(WIDTH, HEIGHT)?, 0)?;
    let frame1 = make_frame(&make_rgha_pixel_buffer(WIDTH, HEIGHT)?, 1)?;
    let frame2 = make_frame(&make_rgha_pixel_buffer(WIDTH, HEIGHT)?, 2)?;

    let nv12_frame0 = make_frame(&make_420v_pixel_buffer(WIDTH, HEIGHT)?, 0)?;
    let nv12_frame1 = make_frame(&make_420v_pixel_buffer(WIDTH, HEIGHT)?, 1)?;

    if caps.super_resolution {
        let scales = super_resolution_supported_scale_factors();
        println!("super-resolution scale factors: {scales:?}");
        if let Some(scale) = scales.first().copied() {
            match super_resolution_model_status(WIDTH, HEIGHT, scale as usize, false, true) {
                Some(SuperResolutionModelStatus::Ready) => {
                    let dst =
                        make_rgha_pixel_buffer(WIDTH * scale as usize, HEIGHT * scale as usize)?;
                    let dst_frame = make_frame(&dst, 0)?;
                    let processor = FrameProcessor::start_super_resolution(
                        WIDTH,
                        HEIGHT,
                        scale as usize,
                        false,
                        true,
                    )?;
                    processor.process_super_resolution(
                        &frame0,
                        None,
                        None,
                        None,
                        FrameProcessorSubmissionMode::Random,
                        &dst_frame,
                    )?;
                    println!(
                        "✅ super-resolution processed {}x{} -> {}x{}",
                        WIDTH,
                        HEIGHT,
                        dst.width(),
                        dst.height()
                    );
                }
                state => {
                    println!(
                        "⚠️  skipping super-resolution: model state {state:?}, progress {:?}",
                        super_resolution_model_percentage_available(
                            WIDTH,
                            HEIGHT,
                            scale as usize,
                            false,
                            true,
                        )
                    );
                }
            }
        }
    }

    if caps.motion_blur {
        let dst = make_rgha_pixel_buffer(WIDTH, HEIGHT)?;
        let dst_frame = make_frame(&dst, 1)?;
        let processor = FrameProcessor::start_motion_blur(WIDTH, HEIGHT, false)?;
        processor.process_motion_blur(
            &frame1,
            Some(&frame2),
            Some(&frame0),
            None,
            None,
            50,
            FrameProcessorSubmissionMode::Sequential,
            &dst_frame,
        )?;
        println!("✅ motion blur processed {WIDTH}x{HEIGHT} RGhA frames");
    }

    if caps.frame_rate_conversion {
        let dst = make_rgha_pixel_buffer(WIDTH, HEIGHT)?;
        let destination_frames = [make_frame(&dst, 1)?];
        let processor = FrameProcessor::start_frame_rate_conversion(WIDTH, HEIGHT, false)?;
        processor.process_frame_rate_conversion(
            &frame0,
            Some(&frame1),
            None,
            &[0.5],
            FrameRateConversionSubmissionMode::Sequential,
            &destination_frames,
        )?;
        println!("✅ frame-rate conversion interpolated one frame at phase 0.5");
    }

    if caps.optical_flow {
        let device = MetalDevice::system_default().ok_or("Metal device unavailable")?;
        let queue = device
            .new_command_queue()
            .ok_or("MTLCommandQueue unavailable")?;
        let command_buffer = queue
            .new_command_buffer()
            .ok_or("MTLCommandBuffer unavailable")?;
        let forward = make_rgha_pixel_buffer(WIDTH, HEIGHT)?;
        let backward = make_rgha_pixel_buffer(WIDTH, HEIGHT)?;
        let flow = FrameProcessorOpticalFlow::new(&forward, &backward)?;
        let processor = FrameProcessor::start_optical_flow(WIDTH, HEIGHT)?;
        processor.process_optical_flow_with_command_buffer(
            &command_buffer,
            &frame0,
            &frame1,
            FrameProcessorSubmissionMode::Sequential,
            &flow,
        )?;
        command_buffer.commit();
        command_buffer.wait_until_completed();
        println!("✅ optical flow processed via Metal command buffer");
    }

    if caps.temporal_noise_filter {
        println!(
            "⚠️  skipping temporal noise filter in this smoke example: the current machine only advertises specialised YUV formats, not the simple 420v helper used here"
        );
    }

    if caps.low_latency_frame_interpolation {
        println!(
            "⚠️  skipping low-latency frame interpolation in this smoke example: synthetic zero-filled test surfaces currently trigger unstable runtime behaviour on this machine"
        );
        let _ = (&nv12_frame0, &nv12_frame1);
    }

    if caps.low_latency_super_resolution {
        let scales = low_latency_super_resolution_supported_scale_factors(WIDTH, HEIGHT);
        println!("low-latency super-resolution scale factors: {scales:?}");
        println!(
            "⚠️  skipping low-latency super-resolution in this smoke example: synthetic zero-filled test surfaces currently trigger unstable runtime behaviour on this machine"
        );
    }

    Ok(())
}
