# VideoToolbox coverage audit

Target crate version: `0.20.0`
Audited SDK: `MacOSX26.2.sdk`

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
| `VTCompressionSession` core session lifecycle / encode / prepare / complete | ✅ | Safe `CompressionSession` + builder. |
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
| `VTMotionEstimationSession` | ✅ | Safe session wrapper with synchronous Rust API bridged over Swift async. |
| `VTRAWProcessingSession` | ✅ | Safe session wrapper plus parameter introspection/writeback. |
| `VTFrameProcessor` capability queries | ✅ | Exposes all public pipeline availability checks. |
| `VTFrameProcessor` session start/end | ✅ | All 7 public pipeline configurations wrapped. |
| `VTFrameProcessorFrame` / `VTFrameProcessorOpticalFlow` | ✅ | Safe retained wrappers with IOSurface-backed validation. |
| `VTFrameProcessor` per-frame processing | ✅ | All 7 pipelines have safe processing helpers. |
| `VTFrameProcessor` Metal command-buffer integration | ✅ | All 7 pipelines have `process_*_with_command_buffer` helpers using `apple-metal`. |
| `VTFrameProcessor` configuration-property introspection (`supportedRevisions`, pixel-buffer attributes, min/max dimensions, etc.) | ◑ | Runtime capability/scale/model-state queries are exposed, but not every configuration property is mirrored yet. |
| `VTFrameProcessor` async per-output callback / async-sequence surface | ⏭ | Public in Swift, but not exposed yet; current Rust API provides synchronous wrappers over completion-handler processing. |

## Explicit SDK findings

- `VTHEVCDecoderRegisterCheck` was **not found** in the current public macOS SDK headers or public `VideoToolbox` symbol lists.
- `VTHEVCEncoderRegisterCheck` was **not found** in the current public macOS SDK headers or public `VideoToolbox` symbol lists.
- A literal public `VTColorSpace` type was **not found** in the current public SDK. Public color-space controls are represented instead by compression/decompression property keys and related `CoreMedia` / `CoreVideo` color metadata constants.

## Verification

The coverage surface carried by `0.20.0` was revalidated with:

- `cargo check --all-features`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`

Examples are intended to be runnable as smoke tests; the frame-processor examples now allocate IOSurface-backed buffers and submit real work instead of only starting sessions. On this machine, the smoke example skips super-resolution when model assets are not yet downloaded, skips temporal noise filter because the current runtime only advertises specialised YUV formats not covered by the simple helper, and skips the low-latency processors when synthetic zero-filled test surfaces trigger unstable runtime behaviour.
