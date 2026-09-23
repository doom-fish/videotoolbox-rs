// VTMotionEstimationSession bridge.
//
// Wraps Apple's macOS 26+ motion-estimation API in a Swift surface
// callable from Rust as plain `@_cdecl` C functions. The session is
// fed two `CVPixelBuffer`s and returns a `CVPixelBuffer` of motion
// vectors through the output handler of
// `VTMotionEstimationSessionEstimateMotionVectors`.

import CoreFoundation
import CoreVideo
import Foundation
import VideoToolbox

private final class VTBMotionEstimationResult: @unchecked Sendable {
    private let lock = NSLock()
    private var status: Int32 = 0
    private var infoFlags: UInt32 = 0
    private var motionVectors: CVPixelBuffer?

    func store(status: Int32, infoFlags: UInt32, motionVectors: CVPixelBuffer?) {
        lock.lock()
        defer { lock.unlock() }
        self.status = status
        self.infoFlags = infoFlags
        self.motionVectors = motionVectors
    }

    func take() -> (status: Int32, infoFlags: UInt32, motionVectors: CVPixelBuffer?) {
        lock.lock()
        defer { lock.unlock() }
        let result = (status, infoFlags, motionVectors)
        motionVectors = nil
        return result
    }
}

// MARK: - Estimation

/// Estimate motion vectors between two pixel buffers. Returns the
/// motion-vector `CVPixelBuffer` (retained +1) via `out`, plus any
/// `VTMotionEstimationInfoFlags` in `infoFlagsOut`.
@_cdecl("vtb_motion_session_estimate")
public func vtb_motion_session_estimate(
    _ session: UnsafeMutableRawPointer,
    _ referenceImage: UnsafeMutableRawPointer,
    _ currentImage: UnsafeMutableRawPointer,
    _ frameFlags: UInt32,
    _ infoFlagsOut: UnsafeMutablePointer<UInt32>?,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    out.pointee = nil
    infoFlagsOut?.pointee = 0
    if #available(macOS 26.0, *) {
        let s: __VTMotionEstimationSession = vtb_borrow(session)
        let refImg: CVPixelBuffer = vtb_borrow(referenceImage)
        let curImg: CVPixelBuffer = vtb_borrow(currentImage)
        let sem = DispatchSemaphore(value: 0)
        let result = VTBMotionEstimationResult()
        let submitStatus = __VTMotionEstimationSessionEstimateMotionVectors(
            s,
            refImg,
            curImg,
            __VTMotionEstimationFrameFlags(rawValue: frameFlags),
            nil
        ) { status, infoFlags, _, motionVectors in
            result.store(
                status: status,
                infoFlags: infoFlags.rawValue,
                motionVectors: motionVectors
            )
            sem.signal()
        }
        if submitStatus != 0 {
            return submitStatus
        }
        sem.wait()
        let completed = result.take()
        infoFlagsOut?.pointee = completed.infoFlags
        if let motionVectors = completed.motionVectors {
            out.pointee = vtb_retain(motionVectors)
        }
        return completed.status
    }
    return VTB_NOT_SUPPORTED
}
