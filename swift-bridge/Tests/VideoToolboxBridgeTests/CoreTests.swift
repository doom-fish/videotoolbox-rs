import Dispatch
import XCTest
@testable import VideoToolboxBridge

final class CoreTests: XCTestCase {
    private final class LifetimeProbe {
        private let onDrop: () -> Void

        init(onDrop: @escaping () -> Void) {
            self.onDrop = onDrop
        }

        deinit {
            onDrop()
        }
    }

    func testBlockOnAsyncTimesOutWithoutRetainingCallerOutputStorage() {
        let cancellationObserved = expectation(description: "task observed cancellation")
        let resultDropped = expectation(description: "late result was dropped")
        let start = DispatchTime.now()

        let result = vtb_block_on_async(timeout: .milliseconds(40)) {
            let value: LifetimeProbe = await withCheckedContinuation { continuation in
                DispatchQueue.global().asyncAfter(deadline: .now() + .milliseconds(160)) {
                    continuation.resume(
                        returning: LifetimeProbe {
                            resultDropped.fulfill()
                        }
                    )
                }
            }
            if Task.isCancelled {
                cancellationObserved.fulfill()
            }
            return value
        }

        guard case .timedOut = result else {
            return XCTFail("expected a distinct timeout result")
        }

        let elapsed = DispatchTime.now().uptimeNanoseconds - start.uptimeNanoseconds
        XCTAssertLessThan(elapsed, 500_000_000)
        wait(for: [cancellationObserved, resultDropped], timeout: 1.0)
    }
}
