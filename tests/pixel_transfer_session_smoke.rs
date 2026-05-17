mod common;

use common::{assert_bgra_pixels_equal, fill_bgra_pixels, make_bgra_pixel_buffer};
use videotoolbox::{DownsamplingMode, PixelTransferSession, ScalingMode, VTError};

#[test]
fn pixel_transfer_session_copies_bgra_pixels() -> Result<(), VTError> {
    assert_ne!(PixelTransferSession::type_id(), 0);

    let src = make_bgra_pixel_buffer(4, 4);
    let src_pixels: Vec<[u8; 4]> = (0..16)
        .map(|index| {
            let value = u8::try_from(index * 11).expect("test pixel value should fit in u8");
            [value, value.wrapping_add(1), value.wrapping_add(2), 0xFF]
        })
        .collect();
    fill_bgra_pixels(&src, &src_pixels);

    let dst = make_bgra_pixel_buffer(4, 4);
    let session = PixelTransferSession::new()?;
    let supported = session.supported_property_dictionary()?;
    assert!(
        !supported.is_empty(),
        "transfer session should publish supported properties"
    );

    session.set_real_time(true)?;
    session.set_scaling_mode(ScalingMode::Normal)?;
    session.set_downsampling_mode(DownsamplingMode::Average)?;
    session.transfer(&src, &dst)?;

    assert_bgra_pixels_equal(&src, &dst);
    Ok(())
}
