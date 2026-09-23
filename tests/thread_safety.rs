use videotoolbox::{
    DecompressionSession, FrameSilo, MultiPassStorage, PixelRotationSession, PixelTransferSession,
};

const fn assert_send<T: Send>() {}

trait AmbiguousIfSync<A> {
    fn check() {}
}

impl<T: ?Sized> AmbiguousIfSync<()> for T {}
impl<T: ?Sized + Sync> AmbiguousIfSync<u8> for T {}

#[test]
fn non_sendable_sdk_sessions_move_between_threads_but_are_not_shared() {
    assert_send::<DecompressionSession>();
    assert_send::<PixelTransferSession>();
    assert_send::<PixelRotationSession>();
    assert_send::<FrameSilo>();
    assert_send::<MultiPassStorage>();
    <DecompressionSession as AmbiguousIfSync<_>>::check();
    <PixelTransferSession as AmbiguousIfSync<_>>::check();
    <PixelRotationSession as AmbiguousIfSync<_>>::check();
    <FrameSilo as AmbiguousIfSync<_>>::check();
    <MultiPassStorage as AmbiguousIfSync<_>>::check();
}

#[cfg(feature = "compression")]
#[test]
fn compression_session_moves_between_threads_but_is_not_shared() {
    assert_send::<videotoolbox::CompressionSession>();
    <videotoolbox::CompressionSession as AmbiguousIfSync<_>>::check();
}

#[cfg(feature = "frame_processor")]
#[test]
fn raw_processing_session_moves_between_threads_but_is_not_shared() {
    assert_send::<videotoolbox::RawProcessingSession>();
    <videotoolbox::RawProcessingSession as AmbiguousIfSync<_>>::check();
}
