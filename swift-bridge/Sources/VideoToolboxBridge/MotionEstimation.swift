// VTMotionEstimationSession bridge.
//
// Wraps Apple's macOS 26+ motion-estimation API in a Swift surface
// callable from Rust as plain `@_cdecl` C functions. The session is
// fed two `CVReadOnlyPixelBuffer`s and returns a `CVPixelBuffer` of
// motion vectors via the async `motion(of:comparedTo:)` API.

import CoreVideo
import Foundation
import VideoToolbox

// MARK: - Lifecycle

@_cdecl("vtb_motion_session_create")
public func vtb_motion_session_create(
    _ width: UInt32,
    _ height: UInt32,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    if #available(macOS 26.0, *) {
        do {
            let s = try VTMotionEstimationSession(width: width, height: height)
            out.pointee = vtb_retain(s)
            return 0
        } catch {
            out.pointee = nil
            return vtb_status(from: error)
        }
    }
    out.pointee = nil
    return VTB_NOT_SUPPORTED
}

@_cdecl("vtb_motion_session_release")
public func vtb_motion_session_release(_ session: UnsafeMutableRawPointer) {
    if #available(macOS 26.0, *) {
        vtb_release(session, as: VTMotionEstimationSession.self)
    }
}

// MARK: - Estimation

/// Estimate motion vectors between two pixel buffers. Returns the
/// motion-vector `CVPixelBuffer` (retained +1) via `out`.
@_cdecl("vtb_motion_session_estimate")
public func vtb_motion_session_estimate(
    _ session: UnsafeMutableRawPointer,
    _ referenceImage: UnsafeMutableRawPointer,
    _ currentImage: UnsafeMutableRawPointer,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    if #available(macOS 26.0, *) {
        let s: VTMotionEstimationSession = vtb_borrow(session)
        let refImg: CVReadOnlyPixelBuffer = vtb_borrow(referenceImage)
        let curImg: CVReadOnlyPixelBuffer = vtb_borrow(currentImage)
        return vtb_block_on_async(
            work: { try await s.motion(of: refImg, comparedTo: curImg) },
            onSuccess: { motion in
                // `Motion` is a CF typealias for CVPixelBuffer at runtime.
                // Cast through AnyObject + Unmanaged to preserve the +1 retain.
                let opaque = Unmanaged.passRetained(motion as AnyObject).toOpaque()
                out.pointee = opaque
            }
        )
    }
    return VTB_NOT_SUPPORTED
}
