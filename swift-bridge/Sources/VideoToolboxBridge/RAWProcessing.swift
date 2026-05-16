// VTRAWProcessingSession bridge — only the async-block bits that
// can't be called cleanly from Rust's C FFI live in Swift. All the
// Create/Invalidate/CopyParameters/SetParameters/CompleteFrames C
// entry points are called directly from Rust via the pure-C FFI in
// `src/ffi/mod.rs` (the Swift overlay hides them with
// `CF_REFINED_FOR_SWIFT`, but they remain linkable from C/Rust).

import CoreMedia
import CoreVideo
import Foundation
import VideoToolbox

/// Asynchronously process a single RAW frame, returning the
/// processed `CVPixelBuffer` (retained +1) via `out`.
@_cdecl("vtb_raw_session_process_frame")
public func vtb_raw_session_process_frame(
    _ session: UnsafeMutableRawPointer,
    _ inputPixelBuffer: UnsafeMutableRawPointer,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    if #available(macOS 15.0, *) {
        let s: VTRAWProcessingSession = vtb_borrow(session)
        let pb: CVPixelBuffer = vtb_borrow(inputPixelBuffer)
        return vtb_block_on_async(
            work: { try await s.process(frame: pb) },
            onSuccess: { processed in
                out.pointee = vtb_retain(processed)
            }
        )
    }
    return VTB_NOT_SUPPORTED
}
