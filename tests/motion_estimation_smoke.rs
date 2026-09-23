#![cfg(all(feature = "frame_processor", feature = "compression"))]

use apple_cf::cf::{AsCFType, CFDictionary, CFNumber, CFString};
use apple_cf::cv::CVPixelBuffer;
use videotoolbox::motion_estimation::MotionEstimationSession;
use videotoolbox::{Codec, CompressionSession, VTError};

const NV12: i64 = 0x3432_3076;

fn surface_backed_nv12_frames(count: usize) -> Vec<CVPixelBuffer> {
    let key = CFString::new("PixelFormatType");
    let format = CFNumber::from_i64(NV12);
    let attributes = CFDictionary::from_pairs(&[(&key as &dyn AsCFType, &format as &dyn AsCFType)]);
    let encoder = CompressionSession::builder(64, 64, Codec::H264)
        .with_source_pixel_buffer_attributes(attributes)
        .build()
        .expect("encoder with an NV12 input pool");
    let pool = encoder.pixel_buffer_pool().expect("input pool");
    (0..count)
        .map(|_| pool.create_pixel_buffer().expect("pool buffer"))
        .collect()
}

#[test]
fn motion_estimation_returns_motion_vectors_through_the_bridge() {
    let session = match MotionEstimationSession::new(64, 64) {
        Ok(session) => session,
        Err(VTError::Unsupported { .. } | VTError::SessionCreateFailed(_)) => return,
        Err(error) => panic!("unexpected error {error:?}"),
    };
    let frames = surface_backed_nv12_frames(2);

    let result = session
        .estimate_with_options(&frames[0], &frames[1], 0)
        .expect("motion vectors");
    assert!(result.motion_vectors.width() > 0);
    assert!(result.motion_vectors.height() > 0);

    let again = session
        .estimate(&frames[1], &frames[0])
        .expect("second estimate");
    assert!(again.width() > 0);
    session.complete_frames().expect("complete");
}
