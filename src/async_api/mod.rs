//! Async API for `videotoolbox`.
//!
//! Provides executor-agnostic [`Future`] wrappers around the crate's one-shot
//! frame submission callbacks. Enable with the `async` cargo feature.
//!
//! ## Available types
//!
//! | Type | Wrapped surface |
//! |------|-----------------|
//! | [`AsyncCompressionSession`] | single-frame `VTCompressionSession` completion |
//! | [`AsyncDecompressionSession`] | single-frame `VTDecompressionSession` completion |
//!
//! ## Tier-2 deferrals
//!
//! The following callback-shaped APIs are intentionally **not** exposed as
//! one-shot futures here:
//!
//! * [`crate::DecompressionSession::set_multi_image_callback`] — multi-fire
//!   callback stream, not a one-shot completion.
//! * [`crate::FrameSilo::sample_buffers`] — synchronous collection helper.
//! * [`crate::MultiPassStorage::close`] and time-range configuration APIs —
//!   synchronous control surfaces.
//!
//! ## Example
//!
//! ```rust,no_run
//! use apple_cf::cm::CMTime;
//! use apple_cf::cv::CVPixelBuffer;
//! use videotoolbox::{Codec, CompressionSession, async_api::AsyncCompressionSession};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! pollster::block_on(async {
//!     let session = CompressionSession::builder(64, 64, Codec::H264)
//!         .with_real_time(true)
//!         .with_average_bit_rate(500_000)
//!         .with_expected_frame_rate(30.0)
//!         .with_max_keyframe_interval(1)
//!         .build()?;
//!     let frame = CVPixelBuffer::create(64, 64, u32::from_be_bytes(*b"BGRA"))
//!         .unwrap_or_else(|status| panic!("CVPixelBuffer::create failed: {status}"));
//!     let sample_buffer = AsyncCompressionSession::new(&session)
//!         .encode_frame(frame, CMTime::new(0, 30), CMTime::INVALID, None)
//!         .await?;
//!     assert!(sample_buffer.is_valid());
//!     Ok::<_, videotoolbox::VTError>(())
//! })?;
//! # Ok(())
//! # }
//! ```

#![cfg(feature = "async")]

use core::future::Future;

use apple_cf::cm::CMSampleBuffer;
use apple_cf::cv::CVImageBuffer;

#[cfg(feature = "compression")]
use apple_cf::{cf::CFDictionary, cm::CMTime, cv::CVPixelBuffer};

#[cfg(feature = "compression")]
use crate::compression::CompressionSession;
use crate::{DecompressionSession, VTError};

/// Async accessor for [`CompressionSession`].
///
/// Wraps the encoder's one-shot frame completion callback as a future that can
/// be awaited on any executor.
#[cfg(feature = "compression")]
#[cfg_attr(docsrs, doc(cfg(all(feature = "async", feature = "compression"))))]
pub struct AsyncCompressionSession<'a> {
    session: &'a CompressionSession,
}

#[cfg(feature = "compression")]
impl core::fmt::Debug for AsyncCompressionSession<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsyncCompressionSession")
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "compression")]
impl<'a> AsyncCompressionSession<'a> {
    /// Wrap a borrowed [`CompressionSession`].
    #[must_use]
    pub const fn new(session: &'a CompressionSession) -> Self {
        Self { session }
    }

    /// Submit `image_buffer` for encoding and await the encoded sample buffer.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] if the frame submission is rejected or
    /// [`VTError::EncoderCallback`] if the encoder callback reports a failure or
    /// drops the frame without a `CMSampleBuffer`.
    #[must_use = "futures do nothing unless awaited"]
    #[allow(clippy::future_not_send)]
    pub fn encode_frame(
        &self,
        image_buffer: CVPixelBuffer,
        presentation_timestamp: CMTime,
        duration: CMTime,
        frame_properties: Option<CFDictionary>,
    ) -> impl Future<Output = Result<CMSampleBuffer, VTError>> + '_ {
        self.session.encode_frame_async(
            image_buffer,
            presentation_timestamp,
            duration,
            frame_properties,
        )
    }
}

#[cfg(feature = "compression")]
impl<'a> From<&'a CompressionSession> for AsyncCompressionSession<'a> {
    fn from(session: &'a CompressionSession) -> Self {
        Self::new(session)
    }
}

/// Async accessor for [`DecompressionSession`].
///
/// Wraps the decoder's one-shot frame completion callback as a future that can
/// be awaited on any executor.
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncDecompressionSession<'a> {
    session: &'a DecompressionSession,
}

impl core::fmt::Debug for AsyncDecompressionSession<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsyncDecompressionSession")
            .finish_non_exhaustive()
    }
}

impl<'a> AsyncDecompressionSession<'a> {
    /// Wrap a borrowed [`DecompressionSession`].
    #[must_use]
    pub const fn new(session: &'a DecompressionSession) -> Self {
        Self { session }
    }

    /// Submit `sample_buffer` for decoding and await the decoded image buffer.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncoderCallback`] if the decoder rejects the frame,
    /// reports an asynchronous failure, or completes without an image buffer.
    #[must_use = "futures do nothing unless awaited"]
    #[allow(clippy::future_not_send)]
    pub fn decode_frame(
        &self,
        sample_buffer: CMSampleBuffer,
        frame_flags: crate::ffi::VTDecodeFrameFlags,
    ) -> impl Future<Output = Result<CVImageBuffer, VTError>> + '_ {
        self.session.decode_frame_async(sample_buffer, frame_flags)
    }
}

impl<'a> From<&'a DecompressionSession> for AsyncDecompressionSession<'a> {
    fn from(session: &'a DecompressionSession) -> Self {
        Self::new(session)
    }
}
