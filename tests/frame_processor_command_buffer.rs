#![cfg(feature = "frame_processor")]

use apple_cf::{cm::CMTime, cv::CVPixelBuffer, iosurface::IOSurface};
use apple_metal::{CommandBufferError, MetalDevice};
use videotoolbox::{
    frame_processor_capabilities, FrameProcessor, FrameProcessorFrame, FrameProcessorOpticalFlow,
    FrameProcessorSubmissionMode, VTError,
};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const VT_FRAME_PROCESSOR_INITIALIZATION_FAILED: i32 = -19736;

fn rgha_pixel_buffer() -> CVPixelBuffer {
    let bytes_per_row = WIDTH * 8;
    let surface = IOSurface::create_with_properties(
        WIDTH,
        HEIGHT,
        u32::from_be_bytes(*b"RGhA"),
        8,
        bytes_per_row,
        bytes_per_row * HEIGHT,
        None,
    )
    .expect("RGhA IOSurface");
    CVPixelBuffer::create_with_io_surface(&surface).expect("pixel buffer")
}

fn frame(index: i64) -> FrameProcessorFrame {
    FrameProcessorFrame::new(&rgha_pixel_buffer(), CMTime::new(index, 60)).expect("frame")
}

#[test]
fn optical_flow_refuses_command_buffers_metal_would_abort_on() {
    if !frame_processor_capabilities().optical_flow {
        eprintln!("skipping: this machine has no VTFrameProcessor optical flow");
        return;
    }
    let device = MetalDevice::system_default().expect("Metal device");
    let queue = device.new_command_queue().expect("command queue");
    let (source, next) = (frame(0), frame(1));
    let flow = FrameProcessorOpticalFlow::new(&rgha_pixel_buffer(), &rgha_pixel_buffer())
        .expect("optical flow");
    let processor = match FrameProcessor::start_optical_flow(WIDTH, HEIGHT) {
        Err(VTError::SessionCreateFailed(VT_FRAME_PROCESSOR_INITIALIZATION_FAILED)) => {
            eprintln!("skipping: VTFrameProcessor reports optical flow but cannot initialize it");
            return;
        }
        other => other.expect("processor"),
    };
    let process = |command_buffer| {
        processor.process_optical_flow_with_command_buffer(
            command_buffer,
            &source,
            &next,
            FrameProcessorSubmissionMode::Sequential,
            &flow,
        )
    };

    let committed = queue.new_command_buffer().expect("command buffer");
    committed.commit().expect("commit");
    committed.wait_until_completed().expect("wait");
    assert!(matches!(
        process(&committed),
        Err(VTError::CommandBuffer(
            CommandBufferError::InvalidState { .. }
        ))
    ));

    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let encoder = command_buffer
        .new_blit_command_encoder()
        .expect("blit encoder");
    assert!(matches!(
        process(&command_buffer),
        Err(VTError::CommandBuffer(CommandBufferError::ActiveEncoder))
    ));
    encoder.end_encoding().expect("end encoding");

    process(&command_buffer).expect("process after the encoder ended");
    command_buffer.commit().expect("commit");
    command_buffer.wait_until_completed().expect("wait");
}
