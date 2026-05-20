#![cfg(all(feature = "async", feature = "decompression"))]

mod common;

use common::{encode_h264_test_frame, H264_AVC1};
use videotoolbox::{async_api::AsyncDecompressionSession, DecompressionSession, VTError};

#[test]
fn async_api_decode_frame_returns_decoded_image_buffer() -> Result<(), VTError> {
    pollster::block_on(async {
        let encoded = encode_h264_test_frame(64, 64)?;
        let sample_buffer = encoded
            .cm_sample_buffer()
            .cloned()
            .expect("encoded frame should expose a CMSampleBuffer");
        let format = sample_buffer
            .format_description()
            .expect("compressed frame should carry a format description");
        assert_eq!(format.media_subtype().as_u32(), H264_AVC1);

        let session = DecompressionSession::new(&format, |_frame| {})?;
        let image_buffer = AsyncDecompressionSession::new(&session)
            .decode_frame(sample_buffer, 0)
            .await?;
        let size = image_buffer.encoded_size();
        assert!((size.width - 64.0).abs() < f64::EPSILON);
        assert!((size.height - 64.0).abs() < f64::EPSILON);

        Ok(())
    })
}
