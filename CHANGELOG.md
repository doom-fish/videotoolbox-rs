# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.11.0] - 2026-05-16

### Added

- `FrameProcessorFrame` and `FrameProcessorOpticalFlow` wrappers for the
  public `VTFrameProcessorFrame` / `VTFrameProcessorOpticalFlow` classes,
  including IOSurface-backed validation on construction.
- Full `VTFrameProcessor` submission helpers for all supported pipelines:
  super-resolution, motion blur, temporal noise filter, frame-rate
  conversion, low-latency super-resolution, low-latency frame
  interpolation, and optical flow.
- Metal command-buffer integration for `VTFrameProcessor` via
  `apple-metal`, so callers can queue work with
  `process_*_with_command_buffer` and synchronize on an existing
  `MTLCommandBuffer`.
- Runtime queries for super-resolution model status / download progress and
  low-latency super-resolution supported scale factors.
- Additional safe wrappers around `VTSession`, compression, decompression,
  pixel-transfer, pixel-rotation, frame-silo, and multi-pass storage entry
  points that were already public in Apple's headers.

### Changed

- Updated `apple-cf` / `apple-metal` dependency ranges to track the local
  `0.6.x` crates used by the rest of the doom-fish stack.
- Reworked the frame-processor examples to submit real frames instead of
  only starting sessions.
- Refreshed crate docs and exports to reflect the optional Swift bridge and
  broader public API surface.

## [0.10.0] - 2026-05-16

### Added

- **`MotionEstimationSession`** (`VTMotionEstimationSession`,
  macOS 26+) — between-frame motion-vector estimation. Async
  `motion(of:comparedTo:)` is wrapped in a synchronous Rust API
  via the Swift bridge; all other entry points (create/invalidate/
  copy-source-attrs/complete-frames) use direct C FFI.
- **`RawProcessingSession`** (`VTRAWProcessingSession`, macOS 15+) —
  ProRes RAW / CinemaDNG decoder. `process(frame:)` runs on the
  Swift async path; `parameters()` returns a fully-typed
  `Vec<RawProcessingParameter>` with key / name / description /
  value-type / min / max / current / initial / camera / neutral
  values pulled from the underlying `CFDictionary`. Includes a
  raw `set_parameters_raw()` writeback path.
- **`FrameProcessor`** session wrapper exposing all 7 pipelines:
  `start_super_resolution`, `start_motion_blur`,
  `start_temporal_noise_filter`, `start_frame_rate_conversion`,
  `start_low_latency_super_resolution`,
  `start_low_latency_frame_interpolation`, `start_optical_flow`.
- Swift bridge restructured into 4 files (`Core.swift`,
  `MotionEstimation.swift`, `RAWProcessing.swift`,
  `FrameProcessor.swift`) following the `screencapturekit-rs`
  pattern — Swift handles complexity (async/throws, configuration
  classes), Rust gets clean ergonomic types.
- New example `08_frame_processor_pipelines` exercises every
  pipeline on M-series hardware.

### Changed

- Swift bridge build now compiles 4 source files instead of 1; the
  `frame_processor` feature still gates the entire bridge so
  encoder-only users pay zero overhead.

## [0.9.0] - 2026-05-16

### Added

- **`VTFrameSilo`** — multi-pass encoder sample-buffer storage
  (`FrameSilo::new()`, `add_sample_buffer()`,
  `progress_of_current_pass()`).
- **`VTMultiPassStorage`** — encoder-private multi-pass scratch
  storage (`MultiPassStorage::new()`); pass `as_ptr()` to a
  `CompressionSession::set_property` with
  `kVTCompressionPropertyKey_MultiPassStorage`.
- **`VTHDRPerFrameMetadataGenerationSession`** — Dolby Vision per-
  frame HDR metadata generation (`HdrMetadataSession::new(fps)`,
  `attach_metadata()`). macOS 15+.
- **`VTUtilities`** — `create_cg_image_from_pixel_buffer()` to turn
  a decoded `CVPixelBuffer` into a `CGImageRef`.
- **`VTProfessionalVideoWorkflow`** —
  `register_professional_workflow_decoders()` and
  `…_encoders()` (extra high-bit-depth ProRes support, etc.).
- New `CMTimeRange` FFI struct alongside `CMTime`.
- New example: `07_multipass_hdr` exercises all four surfaces.

## [0.8.0] - 2026-05-15

### Added

- **`VTFrameProcessor` capability queries** (Swift bridge, opt-in
  via `frame_processor` feature) — runtime detection of super-
  resolution, motion blur, temporal noise filter, frame-rate
  conversion, optical flow, and low-latency variants.
- `super_resolution_supported_scale_factors()` returns the exact
  upscale factors the system advertises.

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
