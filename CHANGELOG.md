# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-15

### Changed (BREAKING)

- `EncodedFrame::cm_sample_buffer_ptr()` returns a raw `*mut c_void` (kept
  for direct extern-"C" hand-off); the new ergonomic accessor
  **`EncodedFrame::cm_sample_buffer()`** returns `Option<&apple_cf::cm::CMSampleBuffer>`.
  Downstream crates should prefer the safe form.
- `EncodedFrame` now owns an `Option<apple_cf::cm::CMSampleBuffer>` instead
  of a manually-retained raw pointer; retain/release moves into apple-cf.

### Added

- `apple-cf` as a regular dependency (with `cm` + `iosurface` features).

## [Unreleased]

### Added

- Initial scaffold targeting `VTCompressionSession`.
- `Codec` enum (H.264, HEVC, ProRes 422/422-HQ/422-LT/422-Proxy/4444).
- `CompressionSessionBuilder` with builder fns for real-time mode, bitrate,
  expected frame rate, frame reordering, max keyframe interval.
- `CompressionSession::encode` accepts an `apple_cf::iosurface::IOSurface`
  directly — zero-copy hand-off from screencapturekit-rs / camera output.
- `EncodedFrame` carries the bitstream bytes, presentation timestamp, and
  encoder info flags.
- `VTError` enum mapping every fallible call site to a distinct variant.
- Pure C `extern "C"` bindings — no Swift bridge, no `bindgen`, no procedural
  macros. Single dependency: `apple-cf`.
- Smoke-test examples that produce real H.264 bitstreams from BGRA IOSurfaces:
  - `01_encode_smoke` — single 1920×1080 frame.
  - `02_encode_sequence` — 30-frame 640×480 sequence with verified IDR pacing.

### Planned

- `VTDecompressionSession` (decoder)
- `VTPixelTransferSession` (pixel-format / colour-space conversion)
- Async encode API via `VTCompressionSessionEncodeFrameWithOutputHandler`
- HEVC profile-level helpers
- HDR metadata
