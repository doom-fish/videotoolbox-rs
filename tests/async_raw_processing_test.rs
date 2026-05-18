#![cfg(all(feature = "async", feature = "frame_processor"))]

mod common;

use common::{make_test_pixel_buffer, make_video_format_description};
use videotoolbox::{copy_raw_processor_extension_properties, RawProcessingSession, VTError};

#[test]
fn process_frame_async_returns_processed_pixel_buffer_when_supported() -> Result<(), VTError> {
    pollster::block_on(async {
        let input = make_test_pixel_buffer(16, 16);
        let Ok(format) = make_video_format_description(&input) else {
            return Ok(());
        };

        if copy_raw_processor_extension_properties(&format).is_err() {
            return Ok(());
        }

        let session = match RawProcessingSession::new(&format) {
            Ok(session) => session,
            Err(VTError::SessionCreateFailed(_)) => return Ok(()),
            Err(error) => return Err(error),
        };

        let processed = match session.process_frame_async(input).await {
            Ok(processed) => processed,
            Err(VTError::EncodeFailed(_)) => return Ok(()),
            Err(error) => return Err(error),
        };

        assert_eq!(processed.width(), 16);
        assert_eq!(processed.height(), 16);
        Ok(())
    })
}
