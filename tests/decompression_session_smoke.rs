mod common;

use common::{encode_h264_test_frame, H264_AVC1};
use std::{sync::mpsc, time::Duration};
use videotoolbox::{DecompressionSession, VTError};

#[test]
fn decompression_session_round_trips_encoded_h264() -> Result<(), VTError> {
    assert_ne!(DecompressionSession::type_id(), 0);

    let encoded = encode_h264_test_frame(64, 64)?;
    let sample_buffer = encoded
        .cm_sample_buffer()
        .expect("encoded frame should expose a CMSampleBuffer");
    let format = sample_buffer
        .format_description()
        .expect("compressed frame should carry a format description");
    assert_eq!(format.media_subtype().as_u32(), H264_AVC1);

    let (tx, rx) = mpsc::channel();
    let session = DecompressionSession::new(&format, move |frame| {
        tx.send(frame).expect("decoder callback receiver dropped");
    })?;

    let supported = session.supported_property_dictionary()?;
    assert!(
        !supported.is_empty(),
        "decoder should publish supported properties"
    );

    session.set_real_time(true)?;
    assert!(unsafe { session.can_accept_format(format.as_ptr().cast()) });

    let _submit_flags = session.decode_with_options(sample_buffer, 0, None)?;
    session.finish_delayed_frames()?;
    session.wait_for_async_frames()?;

    let frame = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("decoder callback timed out");
    assert_eq!(frame.status, 0, "decoder callback should report success");
    assert_eq!(frame.presentation_time, (0, 30));

    let image_buffer = frame
        .image_buffer
        .expect("decoder should yield an image buffer for H.264");
    assert_eq!(image_buffer.width(), 64);
    assert_eq!(image_buffer.height(), 64);

    let black = session.copy_black_pixel_buffer()?;
    assert_eq!(black.width(), 64);
    assert_eq!(black.height(), 64);

    Ok(())
}
