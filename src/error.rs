//! Errors produced by `VideoToolbox` APIs.

use core::fmt;

use crate::ffi::OSStatus;

/// Top-level error returned by all fallible APIs in this crate.
///
/// Wraps Apple's `OSStatus` and tags the call site so the user can tell
/// "the encoder failed to create" apart from "the encoder rejected this frame".
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum VTError {
    /// `VTCompressionSessionCreate` returned non-zero.
    SessionCreateFailed(OSStatus),
    /// `VTSessionSetProperty` returned non-zero. The String identifies which property.
    SetPropertyFailed { key: String, status: OSStatus },
    /// `VTCompressionSessionPrepareToEncodeFrames` returned non-zero.
    PrepareFailed(OSStatus),
    /// `VTCompressionSessionEncodeFrame` returned non-zero.
    EncodeFailed(OSStatus),
    /// `VTCompressionSessionCompleteFrames` returned non-zero.
    CompleteFailed(OSStatus),
    /// `CVPixelBufferCreateWithIOSurface` returned non-zero or NULL.
    PixelBufferCreateFailed(i32),
    /// The user-supplied async callback reported a non-zero status when
    /// encoding a frame.
    EncoderCallback(OSStatus),
    /// An invalid argument was supplied (e.g. zero width).
    InvalidArgument(String),
}

impl VTError {
    /// Underlying `OSStatus` if the error originated from a VT call.
    #[must_use]
    pub const fn status(&self) -> Option<OSStatus> {
        match self {
            Self::SessionCreateFailed(s)
            | Self::SetPropertyFailed { status: s, .. }
            | Self::PrepareFailed(s)
            | Self::EncodeFailed(s)
            | Self::CompleteFailed(s)
            | Self::EncoderCallback(s) => Some(*s),
            Self::PixelBufferCreateFailed(_) | Self::InvalidArgument(_) => None,
        }
    }
}

impl fmt::Display for VTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionCreateFailed(s) => write!(f, "VTCompressionSessionCreate failed: {s}"),
            Self::SetPropertyFailed { key, status } => {
                write!(f, "VTSessionSetProperty({key:?}) failed: {status}")
            }
            Self::PrepareFailed(s) => {
                write!(f, "VTCompressionSessionPrepareToEncodeFrames failed: {s}")
            }
            Self::EncodeFailed(s) => write!(f, "VTCompressionSessionEncodeFrame failed: {s}"),
            Self::CompleteFailed(s) => write!(f, "VTCompressionSessionCompleteFrames failed: {s}"),
            Self::PixelBufferCreateFailed(s) => {
                write!(f, "CVPixelBufferCreateWithIOSurface failed: {s}")
            }
            Self::EncoderCallback(s) => write!(f, "encoder callback reported status {s}"),
            Self::InvalidArgument(m) => write!(f, "invalid argument: {m}"),
        }
    }
}

impl std::error::Error for VTError {}
