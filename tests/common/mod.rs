#![allow(dead_code)]

use apple_cf::{
    cm::{CMBlockBuffer, CMFormatDescription, CMSampleBuffer, CMSampleTimingInfo, CMTime},
    cv::{CVPixelBuffer, CVPixelBufferLockFlags},
    iosurface::{IOSurface, IOSurfaceLockOptions},
};
#[cfg(feature = "compression")]
use videotoolbox::{Codec, CompressionSession, EncodedFrame, VTError};

pub const BGRA: u32 = u32::from_be_bytes(*b"BGRA");
pub const H264_AVC1: u32 = u32::from_be_bytes(*b"avc1");

extern "C" {
    fn CMVideoFormatDescriptionCreateFromH264ParameterSets(
        allocator: videotoolbox::ffi::CFAllocatorRef,
        parameter_set_count: usize,
        parameter_set_pointers: *const *const u8,
        parameter_set_sizes: *const usize,
        nal_unit_header_length: i32,
        format_description_out: *mut videotoolbox::ffi::CMFormatDescriptionRef,
    ) -> i32;
    fn CMSampleBufferCreateReady(
        allocator: videotoolbox::ffi::CFAllocatorRef,
        data_buffer: videotoolbox::ffi::CMBlockBufferRef,
        format_description: videotoolbox::ffi::CMFormatDescriptionRef,
        num_samples: videotoolbox::ffi::CMItemCount,
        num_sample_timing_entries: videotoolbox::ffi::CMItemCount,
        sample_timing_array: *const CMSampleTimingInfo,
        num_sample_size_entries: videotoolbox::ffi::CMItemCount,
        sample_size_array: *const usize,
        sample_buffer_out: *mut videotoolbox::ffi::CMSampleBufferRef,
    ) -> i32;
}

pub fn make_h264_format_description() -> CMFormatDescription {
    const SPS: [u8; 14] = [
        0x67, 0x42, 0x00, 0x1e, 0x95, 0xa8, 0x28, 0x0f, 0x00, 0x44, 0xfc, 0xb8, 0x08, 0x80,
    ];
    const PPS: [u8; 4] = [0x68, 0xce, 0x06, 0xe2];

    let parameter_sets = [SPS.as_ptr(), PPS.as_ptr()];
    let parameter_set_sizes = [SPS.len(), PPS.len()];
    let mut description: videotoolbox::ffi::CMFormatDescriptionRef = core::ptr::null();
    let status = unsafe {
        CMVideoFormatDescriptionCreateFromH264ParameterSets(
            videotoolbox::ffi::kCFAllocatorDefault,
            parameter_sets.len(),
            parameter_sets.as_ptr(),
            parameter_set_sizes.as_ptr(),
            4,
            &mut description,
        )
    };
    assert_eq!(status, 0, "failed to create synthetic H.264 format");
    unsafe { CMFormatDescription::from_raw(description.cast_mut().cast()) }
        .expect("synthetic H.264 format must be non-null")
}

pub fn make_two_sample_buffer() -> CMSampleBuffer {
    let data = CMBlockBuffer::create(&[0, 0]).expect("failed to create synthetic block buffer");
    let timing =
        CMSampleTimingInfo::with_times(CMTime::new(1, 30), CMTime::new(0, 30), CMTime::INVALID);
    let sample_sizes = [1, 1];
    let mut sample_buffer = core::ptr::null_mut();
    let status = unsafe {
        CMSampleBufferCreateReady(
            videotoolbox::ffi::kCFAllocatorDefault,
            data.as_ptr().cast(),
            core::ptr::null_mut(),
            2,
            1,
            core::ptr::from_ref(&timing),
            2,
            sample_sizes.as_ptr(),
            &mut sample_buffer,
        )
    };
    assert_eq!(status, 0, "failed to create synthetic sample buffer");
    unsafe { CMSampleBuffer::from_raw(sample_buffer.cast()) }
        .expect("synthetic sample buffer must be non-null")
}

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
    // SAFETY: Test surfaces are filled before they are submitted to native sessions.
    let bytes = unsafe { guard.as_slice_mut() }.expect("IOSurface must expose bytes");

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

#[cfg(feature = "compression")]
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

pub fn make_test_pixel_buffer(width: usize, height: usize) -> CVPixelBuffer {
    let buffer = make_bgra_pixel_buffer(width, height);
    fill_test_pixel_pattern(&buffer);
    buffer
}

pub fn fill_test_pixel_pattern(buffer: &CVPixelBuffer) {
    let mut pixels = Vec::with_capacity(buffer.width() * buffer.height());
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let seed = y * buffer.width() + x;
            pixels.push([
                u8::try_from(seed % 251).expect("seed must fit in u8"),
                u8::try_from((seed * 3 + 64) % 251).expect("seed must fit in u8"),
                u8::try_from((seed * 7 + 128) % 251).expect("seed must fit in u8"),
                0xFF,
            ]);
        }
    }
    fill_bgra_pixels(buffer, &pixels);
}

pub fn make_video_format_description(
    pixel_buffer: &CVPixelBuffer,
) -> Result<CMFormatDescription, i32> {
    let mut desc: apple_cf::raw::CMVideoFormatDescriptionRef = core::ptr::null();
    let status = unsafe {
        apple_cf::raw::CMVideoFormatDescriptionCreateForImageBuffer(
            apple_cf::raw::kCFAllocatorDefault,
            pixel_buffer.as_ptr().cast(),
            &raw mut desc,
        )
    };
    if status == 0 && !desc.is_null() {
        unsafe { CMFormatDescription::from_raw(desc.cast_mut().cast()) }.ok_or(status)
    } else {
        Err(status)
    }
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
    // SAFETY: Test buffers are filled before they are submitted to native sessions.
    let bytes = unsafe { guard.as_slice_mut() }.expect("read-write lock must expose mutable bytes");

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
    // SAFETY: Processing has completed and the test holds no mutable byte access.
    let bytes = unsafe { guard.as_slice() }.expect("BGRA buffer must be non-planar");
    bytes[offset..offset + 4]
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
    // SAFETY: Processing has completed and both buffers are held read-only.
    let left_bytes = unsafe { left_guard.as_slice() }.expect("lhs BGRA buffer must be non-planar");
    // SAFETY: Processing has completed and both buffers are held read-only.
    let right_bytes =
        unsafe { right_guard.as_slice() }.expect("rhs BGRA buffer must be non-planar");

    for y in 0..left_guard.height() {
        for x in 0..left_guard.width() {
            let left_offset = y * left_guard.bytes_per_row() + x * 4;
            let right_offset = y * right_guard.bytes_per_row() + x * 4;
            assert_eq!(
                &left_bytes[left_offset..left_offset + 4],
                &right_bytes[right_offset..right_offset + 4],
                "pixel mismatch at ({x}, {y})"
            );
        }
    }
}
