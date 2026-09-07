// Core memory-management + FFI utility helpers for the VideoToolbox
// Swift bridge. Mirrors the pattern used by screencapturekit-rs.

import Foundation
import VideoToolbox

// MARK: - Status codes

/// `kVTPropertyNotSupportedErr`
public let VTB_NOT_SUPPORTED: Int32 = -11800

// MARK: - Retained-pointer helpers

/// Take a Swift reference and hand a +1 retained, opaque `void*` to Rust.
@inline(__always)
public func vtb_retain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

/// Borrow a Swift reference from an opaque pointer without changing
/// the retain count.
@inline(__always)
public func vtb_borrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

/// Drop a +1 retained Swift reference that Rust no longer owns.
@inline(__always)
public func vtb_release<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type) {
    Unmanaged<T>.fromOpaque(ptr).release()
}

/// Convert an `NSError` thrown from a Swift call to an `OSStatus`-style
/// `Int32` Rust expects.
@inline(__always)
public func vtb_status(from error: Error) -> Int32 {
    Int32((error as NSError).code)
}

// Private bridge FourCCs: "vtto" for timeout and "vtas" for invalid async state.
let VTB_TIMED_OUT: Int32 = Int32(bitPattern: 0x7674_746f)
private let VTB_ASYNC_STATE_ERROR: Int32 = Int32(bitPattern: 0x7674_6173)

enum VTBBlockingAsyncResult<T> {
    case success(T)
    case failure(Int32)
    case timedOut
}

private final class VTBBlockingAsyncState<T>: @unchecked Sendable {
    private enum Storage {
        case pending
        case completed(VTBBlockingAsyncResult<T>)
        case timedOut
    }

    private let lock = NSLock()
    private var storage: Storage = .pending

    func complete(_ result: VTBBlockingAsyncResult<T>) -> Bool {
        lock.lock()
        defer { lock.unlock() }
        guard case .pending = storage else {
            return false
        }
        storage = .completed(result)
        return true
    }

    func completedResult() -> VTBBlockingAsyncResult<T> {
        lock.lock()
        defer { lock.unlock() }
        guard case let .completed(result) = storage else {
            return .failure(VTB_ASYNC_STATE_ERROR)
        }
        return result
    }

    func markTimedOutOrTakeCompleted() -> VTBBlockingAsyncResult<T>? {
        lock.lock()
        defer { lock.unlock() }
        switch storage {
        case .pending:
            storage = .timedOut
            return nil
        case let .completed(result):
            return result
        case .timedOut:
            return .timedOut
        }
    }
}

/// Synchronously wait for an async Swift call without sharing caller-owned
/// output storage with the task.
func vtb_block_on_async<T>(
    timeout: DispatchTimeInterval = .seconds(30),
    work: @escaping () async throws -> T
) -> VTBBlockingAsyncResult<T> {
    let semaphore = DispatchSemaphore(value: 0)
    let state = VTBBlockingAsyncState<T>()
    let task = Task {
        let result: VTBBlockingAsyncResult<T>
        do {
            result = .success(try await work())
        } catch {
            result = .failure(vtb_status(from: error))
        }
        if state.complete(result) {
            semaphore.signal()
        }
    }

    if semaphore.wait(timeout: .now() + timeout) == .success {
        return state.completedResult()
    }
    if let completed = state.markTimedOutOrTakeCompleted() {
        return completed
    }

    task.cancel()
    return .timedOut
}
