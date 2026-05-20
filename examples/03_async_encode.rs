use std::error::Error;

use apple_cf::cm::CMTime;
use apple_cf::cv::{CVPixelBuffer, CVPixelBufferLockFlags};
use videotoolbox::{async_api::AsyncCompressionSession, Codec, CompressionSession};

const BGRA: u32 = u32::from_be_bytes(*b"BGRA");

fn main() -> Result<(), Box<dyn Error>> {
    pollster::block_on(async {
        let session = CompressionSession::builder(64, 64, Codec::H264)
            .with_real_time(true)
            .with_average_bit_rate(500_000)
            .with_expected_frame_rate(30.0)
            .with_max_keyframe_interval(1)
            .build()?;

        let sample_buffer = AsyncCompressionSession::new(&session)
            .encode_frame(
                make_test_pixel_buffer(64, 64),
                CMTime::new(0, 30),
                CMTime::INVALID,
                None,
            )
            .await?;

        println!("encoded sample buffer valid: {}", sample_buffer.is_valid());
        Ok::<_, videotoolbox::VTError>(())
    })?;

    Ok(())
}

fn make_test_pixel_buffer(width: usize, height: usize) -> CVPixelBuffer {
    let buffer = CVPixelBuffer::create(width, height, BGRA)
        .unwrap_or_else(|status| panic!("CVPixelBuffer::create failed: {status}"));
    fill_test_pixel_pattern(&buffer);
    buffer
}

fn fill_test_pixel_pattern(buffer: &CVPixelBuffer) {
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

fn fill_bgra_pixels(buffer: &CVPixelBuffer, pixels: &[[u8; 4]]) {
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
