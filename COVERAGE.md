# VideoToolbox coverage audit

Target crate version: `0.21.0`
Audited SDK: `MacOSX26.2.sdk` (the surface table was reviewed against `MacOSX26.5.sdk`; the symbol lists in `COVERAGE_AUDIT*.md` were not regenerated)

What the numbers measure: `COVERAGE_AUDIT.md` and `COVERAGE_AUDIT_V2.md` count a symbol as verified when `src/ffi` declares an item with the same name (or, for the Objective-C-only frame processor, motion estimation and RAW processing, when a Swift bridge path reaches it). Most of those rows are raw `extern` declarations without a safe wrapper, so the 100% figure is name coverage of the raw bindings, not safe-API coverage. This file is the safe-API view.

Legend:

- ✅ implemented
- ◑ partial
- ⏭ intentionally skipped / documented

## Header groups audited

- `VTCompressionSession.h`
- `VTCompressionProperties.h`
- `VTDecompressionSession.h`
- `VTDecompressionProperties.h`
- `VTSession.h`
- `VTFrameSilo.h`
- `VTMultiPassStorage.h`
- `VTPixelTransferSession.h`
- `VTPixelTransferProperties.h`
- `VTPixelRotationSession.h`
- `VTPixelRotationProperties.h`
- `VTProfessionalVideoWorkflow.h`
- `VTUtilities.h`
- `VTFrameProcessor*.h`

## Surface summary

| Surface | Status | Notes |
| --- | --- | --- |
| `VTCompressionSession` core session lifecycle / encode / prepare / complete | ✅ | Safe `CompressionSession` + builder: session properties, the encoder specification (hardware preferred/required/disabled, encoder ID, low-latency rate control, preferred/required GPU registry ID), source pixel buffer attributes, and per-frame properties (`FrameProperties`, forced keyframes). Every frame carries its own completion. |
| H.264/HEVC parameter sets | ✅ | Read from the encoded sample buffer's format description with `apple-cf`'s `CMFormatDescription::video_parameter_sets`; not duplicated here. |
| `VTCompressionSession` type/pool/multipass helpers | ✅ | Type ID, pixel-buffer pool, begin/end pass, next-pass time ranges, multipass storage property. |
| Compression property keys / HEVC+H.264 profile levels used by the crate | ✅ | Large public constant set in `src/ffi/mod.rs`; `ProfileLevel` expanded accordingly. |
| Async encode output-handler APIs | ⏭ | Public, but not wrapped yet; current crate keeps the synchronous callback-oriented encode path. |
| `VTDecompressionSession` create/decode/wait/invalidate | ✅ | Safe `DecompressionSession`. |
| `VTDecompressionSessionCopyBlackPixelBuffer` / type ID / `VTIsHardwareDecodeSupported` | ✅ | Safe wrappers plus top-level `is_hardware_decode_supported()`. |
| `VTSession` property copy/set/serializable/supported-property helpers | ✅ | Shared wrappers in `session::`. |
| `VTPixelTransferSession` | ✅ | Transfer, scaling/downsampling/real-time helpers, type ID, session-property helpers. |
| `VTPixelRotationSession` | ✅ | Rotation + flip helpers, type ID, session-property helpers. |
| `VTFrameSilo` | ✅ | Create/add/progress plus next-pass time ranges and sample-buffer iteration collection. |
| `VTMultiPassStorage` | ✅ | Create/type ID/close plus safe attach helper on `CompressionSession`. |
| `VTProfessionalVideoWorkflow` | ✅ | Encoder/decoder registration helpers. |
| `VTUtilities` | ✅ | `VTCreateCGImageFromCVPixelBuffer` plus hardware-decode utility wrapper. |
| `VTMotionEstimationSession` | ✅ | Safe session wrapper (macOS 26); estimation goes through the Swift bridge's output handler. |
| `VTRAWProcessingSession` | ✅ | Safe session wrapper plus parameter introspection/writeback (macOS 15). The parameter-change handler uses `VTRAWProcessingSessionSetParameterChangedHander` on 15.x and the corrected name from 26.0. |
| `VTFrameProcessor` capability queries | ✅ | Exposes all public pipeline availability checks. |
| `VTFrameProcessor` session start/end | ✅ | All 7 public pipeline configurations wrapped. |
| `VTFrameProcessorFrame` / `VTFrameProcessorOpticalFlow` | ✅ | Safe retained wrappers with IOSurface-backed validation. |
| `VTFrameProcessor` per-frame processing | ✅ | All 7 pipelines have safe processing helpers. |
| `VTFrameProcessor` Metal command-buffer integration | ✅ | All 7 pipelines have `process_*_with_command_buffer` helpers using `apple-metal`. |
| `VTFrameProcessor` configuration-property introspection (`supportedRevisions`, pixel-buffer attributes, min/max dimensions, etc.) | ◑ | Runtime capability/scale/model-state queries are exposed, but not every configuration property is mirrored yet. |
| `VTFrameProcessor` async per-output callback / async-sequence surface | ⏭ | Public in Swift, but not exposed yet; current Rust API provides synchronous wrappers over completion-handler processing. |

## Availability

The crate targets macOS 13. Functions and constants introduced in macOS 14, 15 and 26 are resolved with `dlsym` on first use and return `VTError::Unsupported { api, minimum }` when missing, so no safe API imports them strongly; `tests/weak_linking.rs` checks this against the library archive. Plain `DecompressionSession::decode` uses `VTDecompressionSessionDecodeFrame` (macOS 10.8); only frame options need `VTDecompressionSessionDecodeFrameWithOptions` (macOS 15). The Swift bridge weak-links its newer symbols at its macOS 13 deployment target.

## Not wrapped

- macOS 27.0 SDK additions (out of scope for this release): `kVTCompressionPropertyKey_LogTransferFunction`, `kVTProjectionKind_AppleImmersiveVideo`, and `VTLowLatencySuperResolutionScalerConfiguration`'s class-level `maximumDimensions`, `minimumDimensions` and `supportedScaleFactors`.
- `DecompressionSession::set_max_output_buffer_depth` was removed: its `kVTDecompressionPropertyKey_MaximumOutputBufferDepth` key does not exist in any SDK and never linked.

## Explicit SDK findings

- `VTHEVCDecoderRegisterCheck` was **not found** in the current public macOS SDK headers or public `VideoToolbox` symbol lists.
- `VTHEVCEncoderRegisterCheck` was **not found** in the current public macOS SDK headers or public `VideoToolbox` symbol lists.
- A literal public `VTColorSpace` type was **not found** in the current public SDK. Public color-space controls are represented instead by compression/decompression property keys and related `CoreMedia` / `CoreVideo` color metadata constants.

## Verification

The coverage surface carried by `0.21.0` was revalidated with:

- `cargo check --all-features`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`

Examples are intended to be runnable as smoke tests; the frame-processor examples now allocate IOSurface-backed buffers and submit real work instead of only starting sessions. On this machine, the smoke example skips super-resolution when model assets are not yet downloaded, skips temporal noise filter because the current runtime only advertises specialised YUV formats not covered by the simple helper, and skips the low-latency processors when synthetic zero-filled test surfaces trigger unstable runtime behaviour.
