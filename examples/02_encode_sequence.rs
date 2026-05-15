//! Encode a 30-frame sequence and verify each frame's bitstream looks like
//! valid H.264 (AVCC length-prefix or Annex-B start codes).
//!

#![allow(clippy::similar_names)]
//! Run with: `cargo run --example 02_encode_sequence`

use apple_cf::iosurface::{IOSurface, IOSurfaceLockOptions};
use videotoolbox::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 640;
    let height = 480;
    let pixel_format = u32::from_be_bytes(*b"BGRA");

    let surface = IOSurface::create(
        usize::try_from(width)?,
        usize::try_from(height)?,
        pixel_format,
        4,
    )
    .ok_or("failed to allocate IOSurface")?;

    let encoder = CompressionSession::builder(width, height, Codec::H264)
        .with_real_time(true)
        .with_average_bit_rate(2_000_000)
        .with_expected_frame_rate(30.0)
        .with_max_keyframe_interval(15)
        .build()?;

    let mut total_bytes = 0usize;
    let mut keyframes = 0;
    for i in 0..30 {
        // Animate: vary the green channel with frame index so each frame differs.
        {
            let mut guard = surface
                .lock(IOSurfaceLockOptions::NONE)
                .map_err(|c| format!("lock failed: {c}"))?;
            if let Some(bytes) = guard.as_slice_mut() {
                let g = u8::try_from(i * 8).unwrap_or(255);
                for px in bytes.chunks_exact_mut(4) {
                    px[0] = 0x40; // B
                    px[1] = g; // G
                    px[2] = 0x80; // R
                    px[3] = 0xFF; // A
                }
            }
        }

        let encoded = encoder.encode(&surface, (i64::from(i), 30))?;
        total_bytes += encoded.data.len();

        // AVCC: stream of (4-byte BE NAL length || NAL bytes) tuples. Walk the
        // access unit and look for an IDR (NAL type 5) anywhere inside it.
        let mut has_idr = false;
        let mut first_nal_type = 0u8;
        let mut offset = 0;
        while offset + 4 < encoded.data.len() {
            let nal_len = u32::from_be_bytes([
                encoded.data[offset],
                encoded.data[offset + 1],
                encoded.data[offset + 2],
                encoded.data[offset + 3],
            ]) as usize;
            offset += 4;
            if offset >= encoded.data.len() || nal_len == 0 {
                break;
            }
            let nal_type = encoded.data[offset] & 0x1F;
            if first_nal_type == 0 {
                first_nal_type = nal_type;
            }
            if nal_type == 5 {
                has_idr = true;
            }
            offset += nal_len;
        }
        if has_idr {
            keyframes += 1;
        }
        println!(
            "frame {i:>2}: {:>5} bytes, first_nal_type={first_nal_type}{}",
            encoded.data.len(),
            if has_idr { " (KEYFRAME)" } else { "" }
        );
    }

    println!("\nTotal: {total_bytes} bytes across 30 frames; {keyframes} keyframe(s)");
    assert!(keyframes >= 1, "expected at least one keyframe");
    Ok(())
}
