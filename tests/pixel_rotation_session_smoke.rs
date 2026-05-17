mod common;

use apple_cf::cv::CVPixelBufferLockFlags;
use common::{bgra_pixel, make_bgra_pixel_buffer};
use videotoolbox::{PixelRotationSession, Rotation, VTError};

#[test]
fn pixel_rotation_session_rotates_clockwise_90() -> Result<(), VTError> {
    assert_ne!(PixelRotationSession::type_id(), 0);

    let src_width = 1280usize;
    let src_height = 720usize;
    let src = make_bgra_pixel_buffer(src_width, src_height);
    {
        let mut guard = src
            .lock(CVPixelBufferLockFlags::NONE)
            .expect("failed to lock source pixel buffer");
        let bytes_per_row = guard.bytes_per_row();
        let bytes = guard
            .as_slice_mut()
            .expect("read-write lock must expose source bytes");
        bytes.fill(0);

        let corners = [
            ((0usize, 0usize), [10, 20, 30, 0xFF]),
            ((src_width - 1, 0usize), [40, 50, 60, 0xFF]),
            ((0usize, src_height - 1), [70, 80, 90, 0xFF]),
            ((src_width - 1, src_height - 1), [100, 110, 120, 0xFF]),
        ];

        for ((x, y), pixel) in corners {
            let offset = y * bytes_per_row + x * 4;
            bytes[offset..offset + 4].copy_from_slice(&pixel);
        }
    }

    let dst = make_bgra_pixel_buffer(src_height, src_width);
    let session = PixelRotationSession::new()?;
    session.set_rotation(Rotation::Clockwise90)?;
    session.rotate(&src, &dst)?;

    assert_eq!(bgra_pixel(&dst, 0, 0), [70, 80, 90, 0xFF]);
    assert_eq!(bgra_pixel(&dst, src_height - 1, 0), [10, 20, 30, 0xFF]);
    assert_eq!(bgra_pixel(&dst, 0, src_width - 1), [100, 110, 120, 0xFF]);
    assert_eq!(
        bgra_pixel(&dst, src_height - 1, src_width - 1),
        [40, 50, 60, 0xFF]
    );

    Ok(())
}
