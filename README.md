# videotoolbox

Safe, zero-runtime-dependency Rust bindings for Apple's [VideoToolbox](https://developer.apple.com/documentation/videotoolbox) framework — hardware-accelerated H.264, HEVC, and ProRes codecs on macOS.

> **Status:** experimental. Encoder is functional; decoder, pixel transfer, and multi-pass support are planned.

## Features

- **Hardware-accelerated encoding** — H.264, HEVC, and `ProRes` 422/4444
- **Direct `IOSurface` input** — encode zero-copy from screencapturekit / camera output via [`apple-cf::iosurface`](https://github.com/doom-fish/apple-cf-rs)
- **Builder pattern** — fluent configuration of bitrate, frame rate, keyframe interval, real-time mode
- **Pure C bindings** — no Swift bridge, no `bindgen`, no procedural macros
- **Single dependency** — only [`apple-cf`](https://github.com/doom-fish/apple-cf-rs) for shared types

## Why not bindgen?

The full `VideoToolbox` C surface is ~200 symbols, but the useful set for an encoder is closer to 15. Hand-writing those declarations gives us:

- No build-time dependency on `clang`
- Type-safe Rust enums for codec types (instead of raw `u32` four-character codes)
- Builder APIs that map ergonomically to VT's `CFDictionary` property bag

## Requirements

- macOS 13.0+
- Apple Silicon or Intel Mac with hardware video encoder

## Quick start

```rust,no_run
use videotoolbox::prelude::*;
use apple_cf::iosurface::{IOSurface, IOSurfaceLockOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Allocate a 1920×1080 BGRA IOSurface.
    let surface = IOSurface::create(1920, 1080, u32::from_be_bytes(*b"BGRA"), 4)
        .ok_or("failed to allocate")?;

    // Build a real-time H.264 encoder.
    let encoder = CompressionSession::builder(1920, 1080, Codec::H264)
        .with_real_time(true)
        .with_average_bit_rate(8_000_000)
        .with_expected_frame_rate(60.0)
        .with_max_keyframe_interval(120)
        .build()?;

    // Encode one frame and inspect the resulting CMSampleBuffer.
    let encoded = encoder.encode(&surface, (0, 60))?;
    println!("Got {} bytes of H.264", encoded.data.len());

    if let Some(sb) = encoded.cm_sample_buffer() {
        // Hand `sb` straight to avassetwriter::Writer::append_sample for
        // zero-copy muxing — no raw pointer hand-off needed.
        let _ = sb.is_valid();
    }

    Ok(())
}
```

## Composes with the rest of the doom-fish stack

```text
screencapturekit-rs ──► IOSurface ──► videotoolbox-rs ──► H.264 bytes
                                              ↓
                                        avassetwriter-rs (future)
                                              ↓
                                          .mp4 file
```

## Roadmap

- [x] `VTCompressionSession` (encoder)
- [ ] `VTDecompressionSession` (decoder)
- [ ] `VTPixelTransferSession` (pixel format / colour space conversion)
- [ ] `VTMultiPassStorage` (two-pass encoding for offline workflows)
- [ ] Async encode API via `VTCompressionSessionEncodeFrameWithOutputHandler`
- [ ] HEVC profile-level helpers
- [ ] HDR metadata support

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
