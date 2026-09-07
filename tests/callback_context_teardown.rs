#![cfg(feature = "decompression")]

mod common;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::make_h264_format_description;
use videotoolbox::{DecompressionSession, VTError};

struct DropProbe(Arc<AtomicBool>);

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

#[test]
fn decompression_invalidation_releases_callback_capture() -> Result<(), VTError> {
    let dropped = Arc::new(AtomicBool::new(false));
    let probe = DropProbe(Arc::clone(&dropped));
    let format = make_h264_format_description();
    let session = DecompressionSession::new(&format, move |_frame| {
        let _ = &probe;
    })?;

    session.invalidate()?;

    assert!(dropped.load(Ordering::Acquire));
    Ok(())
}

#[test]
fn decompression_drop_releases_callback_capture() -> Result<(), VTError> {
    let dropped = Arc::new(AtomicBool::new(false));
    let probe = DropProbe(Arc::clone(&dropped));
    let format = make_h264_format_description();
    let session = DecompressionSession::new(&format, move |_frame| {
        let _ = &probe;
    })?;

    drop(session);

    assert!(dropped.load(Ordering::Acquire));
    Ok(())
}
