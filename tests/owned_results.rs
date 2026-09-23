mod common;

use common::make_test_pixel_buffer;
use videotoolbox::create_cg_image_from_pixel_buffer;

#[test]
fn cg_image_from_pixel_buffer_is_an_owned_image() {
    let pixel_buffer = make_test_pixel_buffer(32, 16);

    let image = create_cg_image_from_pixel_buffer(&pixel_buffer).expect("BGRA converts");
    let copy = image.clone();
    drop(image);

    assert_eq!(copy.width(), 32);
    assert_eq!(copy.height(), 16);
}

#[cfg(feature = "frame_processor")]
#[test]
fn motion_estimation_source_attributes_are_an_owned_dictionary() {
    use videotoolbox::motion_estimation::MotionEstimationSession;
    use videotoolbox::VTError;

    let session = match MotionEstimationSession::new(64, 64) {
        Ok(session) => session,
        Err(VTError::Unsupported { minimum, .. }) => {
            assert_eq!(minimum, "26.0");
            return;
        }
        Err(VTError::SessionCreateFailed(_)) => return,
        Err(error) => panic!("unexpected error {error:?}"),
    };

    let attributes = session
        .source_pixel_buffer_attributes()
        .expect("attributes are available");
    let copy = attributes.clone();
    drop(attributes);
    assert!(!copy.is_empty());
}
