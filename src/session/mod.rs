//! Video codecs supported by `VideoToolbox`.

use crate::ffi;

/// Video codec families. Maps to `CMVideoCodecType` four-character codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Codec {
    /// H.264 / AVC
    H264,
    /// H.265 / HEVC
    HEVC,
    /// Apple `ProRes` 422 (Standard Definition)
    ProRes422,
    /// Apple `ProRes` 422 HQ
    ProRes422HQ,
    /// Apple `ProRes` 422 LT
    ProRes422LT,
    /// Apple `ProRes` 422 Proxy
    ProRes422Proxy,
    /// Apple `ProRes` 4444 (with alpha)
    ProRes4444,
}

impl Codec {
    /// Lower into a `CMVideoCodecType` for the FFI boundary.
    #[must_use]
    pub const fn as_cm_codec_type(self) -> ffi::CMVideoCodecType {
        match self {
            Self::H264 => ffi::kCMVideoCodecType_H264,
            Self::HEVC => ffi::kCMVideoCodecType_HEVC,
            Self::ProRes422 => ffi::kCMVideoCodecType_AppleProRes422,
            Self::ProRes422HQ => ffi::kCMVideoCodecType_AppleProRes422HQ,
            Self::ProRes422LT => ffi::kCMVideoCodecType_AppleProRes422LT,
            Self::ProRes422Proxy => ffi::kCMVideoCodecType_AppleProRes422Proxy,
            Self::ProRes4444 => ffi::kCMVideoCodecType_AppleProRes4444,
        }
    }
}
