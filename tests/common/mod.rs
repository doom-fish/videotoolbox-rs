#![allow(dead_code)]

use apple_cf::{
    cv::{CVPixelBuffer, CVPixelBufferLockFlags},
    iosurface::{IOSurface, IOSurfaceLockOptions},
};
use videotoolbox::{Codec, CompressionSession, EncodedFrame, VTError};

pub const BGRA: u32 = u32::from_be_bytes(*b"BGRA");
pub const H264_AVC1: u32 = u32::from_be_bytes(*b"avc1");

pub fn make_test_surface(width: usize, height: usize) -> IOSurface {
    let surface = IOSurface::create(width, height, BGRA, 4).expect("failed to allocate IOSurface");
    fill_surface_pattern(&surface);
    surface
}

pub fn fill_surface_pattern(surface: &IOSurface) {
    let mut guard = surface
        .lock(IOSurfaceLockOptions::NONE)
        .expect("failed to lock IOSurface");
    let width = guard.width();
    let height = guard.height();
    let bytes_per_row = guard.bytes_per_row();
    let bytes = guard.as_slice_mut().expect("IOSurface must expose bytes");

    for y in 0..height {
        for x in 0..width {
            let seed = y * width + x;
            let offset = y * bytes_per_row + x * 4;
            bytes[offset..offset + 4].copy_from_slice(&[
                u8::try_from(seed % 251).expect("seed must fit in u8"),
                u8::try_from((seed * 3 + 64) % 251).expect("seed must fit in u8"),
                u8::try_from((seed * 7 + 128) % 251).expect("seed must fit in u8"),
                0xFF,
            ]);
        }
    }
}

pub fn encode_h264_test_frame(width: i32, height: i32) -> Result<EncodedFrame, VTError> {
    let surface = make_test_surface(
        usize::try_from(width).expect("width must be non-negative"),
        usize::try_from(height).expect("height must be non-negative"),
    );
    let session = CompressionSession::builder(width, height, Codec::H264)
        .with_real_time(true)
        .with_average_bit_rate(500_000)
        .with_expected_frame_rate(30.0)
        .with_max_keyframe_interval(1)
        .build()?;
    session.encode(&surface, (0, 30))
}

pub fn make_bgra_pixel_buffer(width: usize, height: usize) -> CVPixelBuffer {
    CVPixelBuffer::create(width, height, BGRA)
        .unwrap_or_else(|status| panic!("CVPixelBuffer::create failed: {status}"))
}

pub fn fill_bgra_pixels(buffer: &CVPixelBuffer, pixels: &[[u8; 4]]) {
    assert_eq!(
        pixels.len(),
        buffer.width() * buffer.height(),
        "expected exactly one BGRA pixel per coordinate"
    );

    let mut guard = buffer
        .lock(CVPixelBufferLockFlags::NONE)
        .expect("failed to lock CVPixelBuffer");
    let width = guard.width();
    let height = guard.height();
    let bytes_per_row = guard.bytes_per_row();
    let bytes = guard
        .as_slice_mut()
        .expect("read-write lock must expose mutable bytes");

    for y in 0..height {
        for x in 0..width {
            let offset = y * bytes_per_row + x * 4;
            bytes[offset..offset + 4].copy_from_slice(&pixels[y * width + x]);
        }
    }
}

pub fn bgra_pixel(buffer: &CVPixelBuffer, x: usize, y: usize) -> [u8; 4] {
    let guard = buffer
        .lock_read_only()
        .expect("failed to lock CVPixelBuffer");
    assert!(x < guard.width(), "x coordinate out of bounds");
    assert!(y < guard.height(), "y coordinate out of bounds");
    let offset = y * guard.bytes_per_row() + x * 4;
    guard.as_slice()[offset..offset + 4]
        .try_into()
        .expect("BGRA pixel should have four bytes")
}

pub fn assert_bgra_pixels_equal(left: &CVPixelBuffer, right: &CVPixelBuffer) {
    assert_eq!(left.width(), right.width(), "pixel-buffer widths differ");
    assert_eq!(left.height(), right.height(), "pixel-buffer heights differ");

    let left_guard = left
        .lock_read_only()
        .expect("failed to lock lhs pixel buffer");
    let right_guard = right
        .lock_read_only()
        .expect("failed to lock rhs pixel buffer");

    for y in 0..left_guard.height() {
        for x in 0..left_guard.width() {
            let left_offset = y * left_guard.bytes_per_row() + x * 4;
            let right_offset = y * right_guard.bytes_per_row() + x * 4;
            assert_eq!(
                &left_guard.as_slice()[left_offset..left_offset + 4],
                &right_guard.as_slice()[right_offset..right_offset + 4],
                "pixel mismatch at ({x}, {y})"
            );
        }
    }
}
