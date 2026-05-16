#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API Documentation
//!
//! Safe, **zero-runtime-dependency** Rust bindings for Apple's
//! [VideoToolbox](https://developer.apple.com/documentation/videotoolbox)
//! framework — hardware-accelerated H.264, HEVC, and ProRes codecs on macOS.
//!
//! Unlike the rest of the doom-fish suite, `VideoToolbox` is a pure C framework,
//! so this crate does **not** ship a Swift bridge — all bindings are direct
//! `extern "C"` declarations against the system framework.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use videotoolbox::prelude::*;
//! use apple_cf::iosurface::IOSurface;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let surface = IOSurface::create(1920, 1080, u32::from_be_bytes(*b"BGRA"), 4)
//!     .ok_or("failed to allocate IOSurface")?;
//!
//! let encoder = CompressionSession::builder(1920, 1080, Codec::H264)
//!     .with_real_time(true)
//!     .with_average_bit_rate(8_000_000)
//!     .with_expected_frame_rate(60.0)
//!     .build()?;
//!
//! let encoded = encoder.encode(&surface, (0, 60))?;
//! println!("Got {} bytes of H.264", encoded.data.len());
//! # Ok(())
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod ffi;
pub mod session;

#[cfg(feature = "compression")]
#[cfg_attr(docsrs, doc(cfg(feature = "compression")))]
pub mod compression;
pub mod decompression;
pub mod encoder_list;
pub mod hdr_metadata;
pub mod multipass;
pub mod transfer;
pub mod utilities;

#[cfg(feature = "frame_processor")]
#[cfg_attr(docsrs, doc(cfg(feature = "frame_processor")))]
pub mod frame_processor;
#[cfg(feature = "frame_processor")]
#[cfg_attr(docsrs, doc(cfg(feature = "frame_processor")))]
pub mod motion_estimation;
#[cfg(feature = "frame_processor")]
#[cfg_attr(docsrs, doc(cfg(feature = "frame_processor")))]
pub mod raw_processing;

pub use error::VTError;
pub use encoder_list::{available_video_encoders, VideoEncoder};
pub use hdr_metadata::HdrMetadataSession;
pub use multipass::{FrameSilo, MultiPassStorage};
pub use session::Codec;
pub use transfer::{PixelRotationSession, PixelTransferSession, Rotation};
pub use utilities::{
    create_cg_image_from_pixel_buffer, register_professional_workflow_decoders,
    register_professional_workflow_encoders,
};

#[cfg(feature = "frame_processor")]
pub use frame_processor::{
    frame_processor_capabilities, super_resolution_supported_scale_factors, FrameProcessor,
    FrameProcessorCapabilities,
};
#[cfg(feature = "frame_processor")]
pub use motion_estimation::MotionEstimationSession;
#[cfg(feature = "frame_processor")]
pub use raw_processing::{RawProcessingParameter, RawProcessingSession};

#[cfg(feature = "compression")]
pub use compression::{CompressionSession, CompressionSessionBuilder, EncodedFrame};

/// Common imports for users of this crate.
pub mod prelude {
    pub use crate::error::VTError;
    pub use crate::session::Codec;
    pub use crate::transfer::{PixelRotationSession, PixelTransferSession, Rotation};

    #[cfg(feature = "compression")]
    pub use crate::compression::{CompressionSession, CompressionSessionBuilder, EncodedFrame};
}
