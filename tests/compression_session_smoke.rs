mod common;

use common::{make_test_surface, H264_AVC1};
use videotoolbox::{Codec, CompressionSession, VTError};

#[test]
fn compression_session_encodes_h264_and_reports_properties() -> Result<(), VTError> {
    assert_ne!(CompressionSession::type_id(), 0);

    let session = CompressionSession::builder(64, 64, Codec::H264)
        .with_real_time(true)
        .with_average_bit_rate(500_000)
        .with_expected_frame_rate(30.0)
        .with_max_keyframe_interval(1)
        .build()?;

    let supported = session.supported_property_dictionary()?;
    assert!(
        !supported.is_empty(),
        "encoder should publish supported properties"
    );

    let surface = make_test_surface(64, 64);
    let encoded = session.encode(&surface, (0, 30))?;
    assert!(!encoded.data.is_empty(), "encoded frame must contain data");

    let sample_buffer = encoded
        .cm_sample_buffer()
        .expect("encoded frame should retain its CMSampleBuffer");
    assert!(sample_buffer.is_valid());
    let format = sample_buffer
        .format_description()
        .expect("encoded frame should publish a format description");
    assert_eq!(format.media_subtype().as_u32(), H264_AVC1);

    Ok(())
}
