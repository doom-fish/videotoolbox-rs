#![cfg(all(feature = "async", feature = "compression"))]

mod common;

use apple_cf::cm::CMTime;
use common::{make_test_pixel_buffer, H264_AVC1};
use videotoolbox::{async_api::AsyncCompressionSession, Codec, CompressionSession, VTError};

#[test]
fn async_api_encode_frame_returns_encoded_sample_buffer() -> Result<(), VTError> {
    pollster::block_on(async {
        let session = CompressionSession::builder(64, 64, Codec::H264)
            .with_real_time(true)
            .with_average_bit_rate(500_000)
            .with_expected_frame_rate(30.0)
            .with_max_keyframe_interval(1)
            .build()?;

        let sample_buffer = AsyncCompressionSession::new(&session)
            .encode_frame(
                make_test_pixel_buffer(64, 64),
                CMTime::new(0, 30),
                CMTime::INVALID,
                None,
            )
            .await?;

        assert!(
            sample_buffer.is_valid(),
            "encoded sample buffer must be valid"
        );
        let format = sample_buffer
            .format_description()
            .expect("encoded frame should publish a format description");
        assert_eq!(format.media_subtype().as_u32(), H264_AVC1);

        Ok(())
    })
}
