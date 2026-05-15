//! Smoke test: allocate a 1920×1080 BGRA `IOSurface`, fill it with a solid
//! colour, and encode a single H.264 frame.
//!
//! Run with: `cargo run --example 01_encode_smoke`
//!
//! Verifies the full Rust → extern "C" → `VideoToolbox` path end-to-end.

use apple_cf::iosurface::{IOSurface, IOSurfaceLockOptions};
use videotoolbox::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 1920;
    let height = 1080;
    let pixel_format = u32::from_be_bytes(*b"BGRA");

    // 1. Allocate an IOSurface.
    let surface = IOSurface::create(
        usize::try_from(width)?,
        usize::try_from(height)?,
        pixel_format,
        4,
    )
    .ok_or("failed to allocate IOSurface")?;

    // 2. Fill it with a solid orange colour (BGRA layout).
    {
        let mut guard = surface
            .lock(IOSurfaceLockOptions::NONE)
            .map_err(|c| format!("lock failed: {c}"))?;
        if let Some(bytes) = guard.as_slice_mut() {
            for px in bytes.chunks_exact_mut(4) {
                px[0] = 0x00; // B
                px[1] = 0x80; // G
                px[2] = 0xFF; // R
                px[3] = 0xFF; // A
            }
        }
    }

    // 3. Build a real-time H.264 encoder targeting 8 Mbps at 60 fps.
    let encoder = CompressionSession::builder(width, height, Codec::H264)
        .with_real_time(true)
        .with_average_bit_rate(8_000_000)
        .with_expected_frame_rate(60.0)
        .with_max_keyframe_interval(120)
        .build()?;

    println!("Encoder ready: 1920×1080 H.264 @ 8 Mbps, 60 fps");

    // 4. Encode the frame.
    let encoded = encoder.encode(&surface, (0, 60))?;
    println!(
        "Encoded frame: {} bytes (info_flags = 0x{:x}, pts = {:?})",
        encoded.data.len(),
        encoded.info_flags,
        encoded.presentation_time
    );

    // Sanity check: any sane H.264 NAL unit should start with 0x00000001 or 0x000001
    // OR be in AVCC length-prefixed form. We just check it's non-empty.
    assert!(!encoded.data.is_empty(), "encoder produced an empty frame");

    Ok(())
}
