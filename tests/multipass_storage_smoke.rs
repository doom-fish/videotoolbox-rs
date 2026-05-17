use videotoolbox::{FrameSilo, MultiPassStorage, VTError};

#[test]
fn multipass_storage_and_frame_silo_create_successfully() -> Result<(), VTError> {
    assert_ne!(FrameSilo::type_id(), 0);
    assert_ne!(MultiPassStorage::type_id(), 0);

    let silo = FrameSilo::new()?;
    assert!(
        !silo.as_ptr().is_null(),
        "frame silo pointer should be non-null"
    );

    let storage = MultiPassStorage::new()?;
    assert!(
        !storage.as_ptr().is_null(),
        "multi-pass storage pointer should be non-null"
    );
    storage.close()?;

    Ok(())
}
