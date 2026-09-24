# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.21.0] - 2026-09-24

### Fixed

- The seven `FrameProcessor::process_*_with_command_buffer` functions aborted
  the process when the command buffer was already committed (`Completed
  handler provided after commit call`) or had an apple-metal encoder open
  (`encodeSignalEvent:value: with uncommitted encoder`), and another thread
  could commit the buffer mid-submission. They now submit inside apple-metal's
  `CommandBuffer::encode_foreign` and return `VTError::CommandBuffer` instead.
- The frame-processor exports took the submission mode as a Swift `Int` while
  Rust passed an `i32`, so the upper half of the register was undefined on
  arm64 and Swift fell back to random submission. Both sides now use a 32-bit
  value, and an unknown mode returns a parameter error.
- Binaries that used the crate failed to load on macOS 13 and 14 although the
  README and Package.swift promise macOS 13: functions and constants from
  macOS 14, 15 and 26 were strong imports, plain `DecompressionSession::decode`
  among them. The safe APIs now resolve them at run time and return
  `VTError::Unsupported { api, minimum }` on older systems; `decode` and
  `decode_with_options(.., None)` use `VTDecompressionSessionDecodeFrame`
  (macOS 10.8). The crate's own test and example binaries weak-link
  VideoToolbox and CoreMedia.
- `CompressionSession::encode` handed out results through one shared channel,
  so a caller could receive another caller's frame and a failed
  `VTCompressionSessionCompleteFrames` left a late frame queued that shifted
  every later result by one. Every frame now has its own completion, keyed by
  `sourceFrameRefCon`.
- `encode_frame_async` could hang forever with the default settings, because
  nothing forced frames held back for reordering out of the encoder. The first
  poll that finds its frame still pending now completes frames up to that
  frame's timestamp.
- A pending `decode_frame_async` never resolved, and leaked its completion,
  when VideoToolbox delivered its output or failure to an installed
  multi-image callback.
- Sessions that the SDK marks non-Sendable were `Sync`, so safe code could
  drive one session from several threads at once.
- `create_cg_image_from_pixel_buffer` and
  `MotionEstimationSession::source_pixel_buffer_attributes` returned raw +1
  pointers from safe functions.
- The Swift bridge bound `VTMotionEstimationSessionEstimateMotionVectors` and
  `VTRAWProcessingSessionSetParameterChangedHandler` with `@_silgen_name`;
  motion estimation also borrowed the CF session as the overlay's Swift class
  and let its output handler write through the caller's out-pointers. Both now
  use the SDK's C declarations, and results are copied out after the wait.
- The RAW parameter-change handler was refused on macOS 15.x; it now uses the
  15.x entry point, `VTRAWProcessingSessionSetParameterChangedHander`.
- Bridge error codes and the super-resolution model status were converted with
  trapping `Int32(_:)` conversions.
- `TaggedBufferGroup::pixel_buffer_at` and `sample_buffer_at` check the index
  against the group's count.
- Docs: `EncodedFrame::data` is AVCC-style length-prefixed NAL units (not
  plain NAL units), the README lists which APIs need which macOS, that
  sessions are `Send` but not `Sync`, and how raw `ffi` users must weak-link;
  `COVERAGE*.md` say that the audit counts names of raw declarations.

### Changed

- **BREAKING:** `CompressionSession`, `DecompressionSession`,
  `RawProcessingSession`, `PixelTransferSession`, `PixelRotationSession`,
  `FrameSilo` and `MultiPassStorage` are `Send` but no longer `Sync`.
- **BREAKING:** timestamps are `apple_cf::cm::CMTime` instead of `(i64, i32)`:
  the `presentation_time` argument of `CompressionSession::encode` and
  `encode_multi_image`, `EncodedFrame::presentation_time`, and
  `presentation_time`/`duration` of `DecodedFrame` and
  `DecodedMultiImageFrame`. Dropped frames report their source timestamp
  instead of `(0, 0)`.
- **BREAKING:** `CompressionSession::encode_frame_async` returns
  `impl Future` and submits the frame when it is called.
- **BREAKING:** `type_id()` of `TaggedBufferGroup`, `HdrMetadataSession`,
  `RawProcessingSession` and `MotionEstimationSession` returns
  `Result<usize, VTError>`.
- **BREAKING:** `create_cg_image_from_pixel_buffer` returns
  `apple_cf::cg::CGImage`, and
  `MotionEstimationSession::source_pixel_buffer_attributes` returns
  `apple_cf::cf::CFDictionary`.
- `doom-fish-utils` is a required dependency; in-family requirements are
  `apple-cf >=0.11, <0.12`, `apple-metal >=0.10, <0.11` and
  `doom-fish-utils >=0.4.1, <0.5`, and `rust-version` is 1.82 (was 1.76).

### Added

- `VTError::CommandBuffer(apple_metal::CommandBufferError)` (with the
  `frame_processor` feature), which is also the error's `source()`.
- Encoder specification options on `CompressionSessionBuilder`:
  `with_hardware_acceleration(HardwareAcceleration::{Preferred, Required,
  Disabled})`, `with_encoder_id`, `with_low_latency_rate_control` and
  `with_encoder_gpu(EncoderGpu::{Preferred, Required})`, plus
  `with_source_pixel_buffer_attributes`.
- `CompressionSession::encode_with_properties` and `FrameProperties` for
  per-frame properties such as forced keyframes; `FrameProperties::to_dictionary`
  builds the dictionary `encode_frame_async` takes.
- `VTError::Unsupported { api, minimum }`.

### Removed

- **BREAKING:** `DecompressionSession::set_max_output_buffer_depth` and
  `ffi::kVTDecompressionPropertyKey_MaximumOutputBufferDepth`. No SDK declares
  that key, so any binary calling the setter failed to link.
- The unused Swift bridge exports `vtb_motion_session_create` and
  `vtb_motion_session_release`.

## [0.20.0] - 2026-09-07

### Added

- `CompressionSession::invalidate` and `DecompressionSession::invalidate`
  explicitly drain callbacks, release native resources, and report a
  quiescence failure that `Drop` cannot surface.

### Changed (breaking)

- Migrated `apple-cf` raw wrappers to distinguish transferred +1 ownership
  from borrowed +0 references, and documented the aliasing invariants around
  unsafe pixel-buffer and `IOSurface` byte views.
- Raised in-family requirements to `apple-cf >=0.10, <0.11`,
  `apple-metal >=0.9, <0.10`, and `doom-fish-utils >=0.4, <0.5`.

### Fixed

- One-shot async decompression now rejects multi-sample `CMSampleBuffer`s with
  `VTError::UnexpectedSampleCount`, preventing repeated consumption of one
  completion context.
- Synchronous RAW processing stores async results in owned Swift state, returns
  `VTError::TimedOut` on its bounded timeout, and never lets a late task write
  through a caller stack pointer.
- RAW parameter-handler replacement now serializes the native transition with
  Rust ownership and retains each installed context until native and in-flight
  block references are gone.
- Successful compression and decompression teardown now reclaims the callback
  reference retained for the native session instead of leaking it.

## [0.19.0] - 2026-08-31

### Added

- `ProfileLevel::ALL`, `ProfileLevel::name()` and `ProfileLevel::from_name()`,
  so encoder profiles can be selected from configuration strings without each
  caller maintaining its own name table. `from_name` ignores case and
  `_`/`-`/`.` separators.
- `ProfileLevel` is now re-exported from the `compression` prelude.

## [0.18.2] - 2026-08-31

### Fixed

- `DecompressionSession` teardown now calls
  `VTDecompressionSessionWaitForAsynchronousFrames` before invalidating the
  session. VideoToolbox dispatches the decode output callback on its own queue
  even when async mode was not requested, so invalidating while a callback was
  in flight freed the callback ref-con underneath the running callback. This
  crashed on the decoder callback queue whenever the decoder was replaced (an
  in-stream SPS/PPS rebuild) or torn down.

## [0.18.1] - 2026-05-20

### Fixed

- Decode callbacks are guarded against panicking across the `extern "C"`
  boundary, and a poisoned mutex no longer aborts the callback.
- Swift/C FFI hardening: panic safety at every trampoline, ABI layout
  assertions, and a single retain/release macro for CoreFoundation handles.

## [0.18.0] - 2026-05-20

### Added

- `AsyncRawProcessingSession` in `videotoolbox::async_api`, mirroring async RAW frame processing and exposing `parameter_changes(...)` as a bounded async stream over `VTRAWProcessingSessionSetParameterChangedHandler`.

### Notes

- Phase 32 completeness + async sweep.
- `VTDecompressionSessionSetMultiImageCallback` remains deferred because the audited C signature requires a non-null callback and does not expose a clear/unsubscribe path for RAII stream teardown.

## [0.17.1] - 2026-05-20

- Added in-`src/` unit tests across `compression`, `error`, `session`, and `transfer` (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.17.0] - 2026-05-20

### Added

- `async_api` module behind the `async` feature, providing executor-agnostic async/await wrappers for callback-based VideoToolbox APIs (compression/decompression frame callbacks). Uses `doom-fish-utils::completion` so callers can use any executor (tokio, async-std, pollster, etc.).
- New example showing the async encode flow.

## [0.16.3] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.16.2] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.16.1] - 2026-05-18

### Changed

- Re-exported `Boolean` and `OSStatus` from `apple_cf::raw`, removing the remaining crate-local primitive aliases.

## [0.16.0] - 2026-05-18

### Changed

- Re-export `CVPixelBufferRef` and `CVPixelBufferPoolRef` from `apple_cf::raw`, removing the remaining crate-local CoreVideo duplicate aliases.

## [0.15.0] - 2026-05-18

### Changed

- Re-export `CMSampleBufferRef`, `CMBlockBufferRef`, `CMFormatDescriptionRef`, `CMTaggedBufferGroupRef`, `CMItemCount`, `CMVideoCodecType`, `CMTimeFlags`, and `CFNumberType` from `apple_cf::raw`, removing the remaining crate-local CoreMedia/CoreFoundation duplicate aliases.

## [0.14.0] - 2026-05-18

### Changed

- Re-export `CFAllocatorRef`, `CFTypeRef`, `CFStringRef`, `CFNumberRef`, `CFBooleanRef`, `CFDictionaryRef`, `CFMutableDictionaryRef`, `CFArrayRef`, and `CFURLRef` from `apple_cf::raw` instead of defining crate-local duplicates. This is a breaking change for code that relied on the old local aliases, and it aligns the FFI surface with the shared CoreFoundation definitions.

## [0.13.1] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.13.0] - 2026-05-18

### Added

- Tier-1 async frame-submission wrappers backed by `doom_fish_utils::AsyncCompletion`:
  `CompressionSession::encode_frame_async`,
  `DecompressionSession::decode_frame_async`, and
  `RawProcessingSession::process_frame_async`.

## [0.12.0] - 2026-05-18

### Changed

- `ffi::CMTime` and `ffi::CMTimeRange` now re-export `apple_cf::cm::{CMTime, CMTimeRange}` instead of defining crate-local duplicates. This is a breaking change for code that relied on the old nominally-distinct FFI types, but it removes cross-crate type mismatches when sharing Core Media time values.

## [0.11.4] - 2026-05-17

### Changed

- Added comprehensive `SAFETY:` documentation comments to all unsafe FFI 
  blocks across the crate, improving correctness verification and code review 
  clarity. Each unsafe block now explains why it is safe to call the underlying 
  Apple SDK function with the given arguments.

## [0.11.3] - 2026-05-17

### Added

- Six new integration smoke tests covering `VTCompressionSession`,
  `VTDecompressionSession`, `VTPixelTransferSession`,
  `VTPixelRotationSession`, `VTFrameProcessor` capability queries, and
  `VTMultiPassStorage` / `VTFrameSilo`.
- Shared `tests/common` fixtures for reusable `IOSurface` and
  `CVPixelBuffer` setup across the new runtime tests.

## [0.11.2] - 2026-05-17

### Added

- Public frame-processor configuration enums/structs for super-resolution,
  motion blur, frame-rate conversion, and optical flow, plus configuration-
  aware session/model helpers.
- HDR metadata type-ID / format-surface coverage via `HdrMetadataSession::type_id`,
  `HdrMetadataFormat`, and `HdrMetadataSession::new_with_formats`.
- Motion-estimation creation options / frame flags / info flags, and RAW
  processing property + parameter-changed-handler wrappers.
- Supplemental decoder + Media Extension property helpers in `utilities`, plus
  crate-root re-exports for the new HDR / utilities / frame-processor /
  motion-estimation surface.
- Audit smoke coverage for the final frame-processor / HDR / motion / RAW /
  VTSession / VTUtilities gaps.

### Changed

- Refreshed `COVERAGE_AUDIT.md` to 100% public SDK coverage (448 verified,
  0 gaps, 1 exempt deprecated alias).

## [0.11.1] - 2026-05-16

### Added

- The remaining public `VTCompressionProperties.h` /
  `VTDecompressionProperties.h` constants called out by the audit, including
  encoder/decoder specification keys, HDR / stereo / calibration keys, and
  per-frame decode option keys.
- Raw FFI coverage for the async / multi-image encode and decode entry points:
  `VTCompressionSessionEncodeFrameWithOutputHandler`,
  `VTCompressionSessionEncodeMultiImageFrame*`,
  `VTDecompressionSessionDecodeFrameWith*`,
  `VTDecompressionSessionSetMultiImageCallback`, and the stereo MV-HEVC support
  queries.
- `TaggedBufferGroup`, `CompressionSession::encode_multi_image`,
  `DecompressionSession::decode_with_options`, and
  `DecompressionSession::set_multi_image_callback` for the new multi-image /
  per-frame-options surface.
- `available_video_encoder_details[_with_options]` and
  `supported_property_dictionary_for_encoder`, exposing the extended
  `VTVideoEncoderList` metadata and selection-property dictionary helpers.
- Audit smoke tests covering the newly-added FFI surface and safe wrappers.

### Changed

- `examples/05_encoder_list` now prints encoder metadata and demonstrates
  `supported_property_dictionary_for_encoder()`.
- Refreshed `COVERAGE_AUDIT.md` after closing the compression/decompression
  constant, async/multi-image, and encoder-list metadata gaps.

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

## [0.1.0] - 2026-05-15

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
