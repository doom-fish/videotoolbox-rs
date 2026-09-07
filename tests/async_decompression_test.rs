#![cfg(all(feature = "async", feature = "decompression"))]

mod common;

#[cfg(feature = "compression")]
use common::{encode_h264_test_frame, H264_AVC1};
use common::{make_h264_format_description, make_two_sample_buffer};
use videotoolbox::{DecompressionSession, VTError};

#[cfg(feature = "compression")]
#[test]
fn decode_frame_async_returns_decoded_image_buffer() -> Result<(), VTError> {
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
        let image_buffer = session.decode_frame_async(sample_buffer, 0).await?;
        let size = image_buffer.encoded_size();
        assert!((size.width - 64.0).abs() < f64::EPSILON);
        assert!((size.height - 64.0).abs() < f64::EPSILON);

        Ok(())
    })
}

#[test]
fn decode_frame_async_rejects_multi_sample_buffers_before_submission() -> Result<(), VTError> {
    pollster::block_on(async {
        let format = make_h264_format_description();
        let session = DecompressionSession::new(&format, |_frame| {})?;
        let sample_buffer = make_two_sample_buffer();
        assert_eq!(sample_buffer.num_samples(), 2);

        let result = session.decode_frame_async(sample_buffer, 0).await;
        assert_eq!(
            result.err(),
            Some(VTError::UnexpectedSampleCount {
                operation: "DecompressionSession::decode_frame_async",
                expected: 1,
                actual: 2,
            })
        );

        session.invalidate()
    })
}
