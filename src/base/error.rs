use core_foundation::base::OSStatus;
use thiserror::Error;

const PROPERTY_NOT_SUPPORTED: OSStatus = -12900;
const PROPERTY_READ_ONLY: OSStatus = -12901;
const PARAMETER: OSStatus = -12902;
const INVALID_SESSION: OSStatus = -12903;
const ALLOCATION_FAILED: OSStatus = -12904;
const PIXEL_TRANSFER_NOT_SUPPORTED: OSStatus = -12905;
const COULD_NOT_FIND_VIDEO_DECODER: OSStatus = -12906;
const COULD_NOT_CREATE_INSTANCE: OSStatus = -12907;
const COULD_NOT_FIND_VIDEO_ENCODER: OSStatus = -12908;
const VIDEO_DECODER_BAD_DATA: OSStatus = -12909;
const VIDEO_DECODER_UNSUPPORTED_DATA_FORMAT: OSStatus = -12910;
const VIDEO_DECODER_MALFUNCTION: OSStatus = -12911;
const VIDEO_ENCODER_MALFUNCTION: OSStatus = -12912;
const VIDEO_DECODER_NOT_AVAILABLE_NOW: OSStatus = -12913;
const PIXEL_ROTATION_NOT_SUPPORTED: OSStatus = -12914;
const VIDEO_ENCODER_NOT_AVAILABLE_NOW: OSStatus = -12915;
const FORMAT_DESCRIPTION_CHANGE_NOT_SUPPORTED: OSStatus = -12916;
const INSUFFICIENT_SOURCE_COLOR_DATA: OSStatus = -12917;
const COULD_NOT_CREATE_COLOR_CORRECTION_DATA: OSStatus = -12918;
const COLOR_SYNC_TRANSFORM_CONVERT_FAILED: OSStatus = -12919;
const VIDEO_DECODER_AUTHORIZATION: OSStatus = -12210;
const VIDEO_ENCODER_AUTHORIZATION: OSStatus = -12211;
const COLOR_CORRECTION_PIXEL_TRANSFER_FAILED: OSStatus = -12212;
const MULTI_PASS_STORAGE_IDENTIFIER_MISMATCH: OSStatus = -12213;
const MULTI_PASS_STORAGE_INVALID: OSStatus = -12214;
const FRAME_SILO_INVALID_TIME_STAMP: OSStatus = -12215;
const FRAME_SILO_INVALID_TIME_RANGE: OSStatus = -12216;
const COULD_NOT_FIND_TEMPORAL_FILTER: OSStatus = -12217;
const PIXEL_TRANSFER_NOT_PERMITTED: OSStatus = -12218;
const COLOR_CORRECTION_IMAGE_ROTATION_FAILED: OSStatus = -12219;
const VIDEO_DECODER_REMOVED: OSStatus = -17690;
const SESSION_MALFUNCTION: OSStatus = -17691;
const VIDEO_DECODER_NEEDS_ROSETTA: OSStatus = -17692;
const VIDEO_ENCODER_NEEDS_ROSETTA: OSStatus = -17693;
const VIDEO_DECODER_REFERENCE_MISSING: OSStatus = -17694;
const VIDEO_DECODER_CALLBACK_MESSAGING: OSStatus = -17695;
const VIDEO_DECODER_UNKNOWN: OSStatus = -17696;
const EXTENSION_DISABLED: OSStatus = -17697;
const VIDEO_ENCODER_MVHEVCVIDEO_LAYER_IDS_MISMATCH: OSStatus = -17698;
const COULD_NOT_OUTPUT_TAGGED_BUFFER_GROUP: OSStatus = -17699;
const COULD_NOT_FIND_EXTENSION: OSStatus = -19510;
const EXTENSION_CONFLICT: OSStatus = -19511;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum VTError<TUnknown = OSStatus> {
    #[error("Property not supported")]
    PropertyNotSupported,
    #[error("Property is read only")]
    PropertyReadOnly,
    #[error("Parameter error")]
    Parameter,
    #[error("Invalid session")]
    InvalidSession,
    #[error("Allocation failed")]
    AllocationFailed,
    #[error("Pixel transfer not supported")]
    PixelTransferNotSupported, // c.f. -8961
    #[error("Could not find video decoder")]
    CouldNotFindVideoDecoder,
    #[error("Could not create instance")]
    CouldNotCreateInstance,
    #[error("Could not find video encoder")]
    CouldNotFindVideoEncoder,
    #[error("Video decoder: Bad data")]
    VideoDecoderBadData, // c.f. -8969
    #[error("Video decoder: Unsupported data format")]
    VideoDecoderUnsupportedDataFormat, // c.f. -8970
    #[error("Video decoder: Malfunction")]
    VideoDecoderMalfunction, // c.f. -8960
    #[error("Video encoder: Malfunction")]
    VideoEncoderMalfunction,
    #[error("Video decoder: Not available now")]
    VideoDecoderNotAvailableNow,
    #[error("Image rotation not supported")]
    ImageRotationNotSupported, // deprecated
    #[error("Pixel rotation not supported")]
    PixelRotationNotSupported,
    #[error("Video encoder: Not available now")]
    VideoEncoderNotAvailableNow,
    #[error("Format description change not supported")]
    FormatDescriptionChangeNotSupported,
    #[error("Insufficient source color data")]
    InsufficientSourceColorData,
    #[error("Could not create color correction data")]
    CouldNotCreateColorCorrectionData,
    #[error("Color sync transform convert failed")]
    ColorSyncTransformConvertFailed,
    #[error("Video decoder: Authorization error")]
    VideoDecoderAuthorization,
    #[error("Video decoder: Authorization error")]
    VideoEncoderAuthorization,
    #[error("Color correction pixel transfer failed")]
    ColorCorrectionPixelTransferFailed,
    #[error("Multi-pass storage identifier mismatch")]
    MultiPassStorageIdentifierMismatch,
    #[error("Multi-pass storage invalid")]
    MultiPassStorageInvalid,
    #[error("Frame silo: Invalid time stamp")]
    FrameSiloInvalidTimeStamp,
    #[error("Frame silo: Invalid time range")]
    FrameSiloInvalidTimeRange,
    #[error("Frame silo: Could not find temporal filter")]
    CouldNotFindTemporalFilter,
    #[error("Could not find temporal filter")]
    PixelTransferNotPermitted,
    #[error("Pixel transfer not permitted")]
    ColorCorrectionImageRotationFailed,
    #[error("Video decoder removed")]
    VideoDecoderRemoved,
    #[error("Session malfunction")]
    SessionMalfunction,
    #[error("Video decoder needs Rosetta")]
    VideoDecoderNeedsRosetta,
    #[error("Video encoder needs Rosetta")]
    VideoEncoderNeedsRosetta,
    #[error("video decoder reference missing")]
    VideoDecoderReferenceMissing,
    #[error("Video decoder callback messaging")]
    VideoDecoderCallbackMessaging,
    #[error("Video decoder unknown")]
    VideoDecoderUnknown,
    #[error("Extension disabled")]
    ExtensionDisabled,
    #[error("Video encoder: MVHEVCVideoLayer ids mismatch")]
    VideoEncoderMVHEVCVideoLayerIdsMismatch,
    #[error("Could not output tagged buffer group")]
    CouldNotOutputTaggedBufferGroup,
    #[error("Could not find extension")]
    CouldNotFindExtension,
    #[error("Extension conflict")]
    ExtensionConflict,
    #[error("An unknown error occurred with code {0}")]
    UnknownError(TUnknown),
}

impl Default for VTError {
    fn default() -> Self {
        Self::UnknownError(-1)
    }
}

pub const NO_ERROR: OSStatus = 0;

impl From<OSStatus> for VTError {
    fn from(value: OSStatus) -> Self {
        match value {
            // complete list of errors
            PROPERTY_NOT_SUPPORTED => VTError::PropertyNotSupported,
            PROPERTY_READ_ONLY => VTError::PropertyReadOnly,
            PARAMETER => VTError::Parameter,
            INVALID_SESSION => VTError::InvalidSession,
            ALLOCATION_FAILED => VTError::AllocationFailed,
            PIXEL_TRANSFER_NOT_SUPPORTED => VTError::PixelTransferNotSupported,
            COULD_NOT_FIND_VIDEO_DECODER => VTError::CouldNotFindVideoDecoder,
            COULD_NOT_CREATE_INSTANCE => VTError::CouldNotCreateInstance,
            COULD_NOT_FIND_VIDEO_ENCODER => VTError::CouldNotFindVideoEncoder,
            VIDEO_DECODER_BAD_DATA => VTError::VideoDecoderBadData,
            VIDEO_DECODER_UNSUPPORTED_DATA_FORMAT => VTError::VideoDecoderUnsupportedDataFormat,
            VIDEO_DECODER_MALFUNCTION => VTError::VideoDecoderMalfunction,
            VIDEO_ENCODER_MALFUNCTION => VTError::VideoEncoderMalfunction,
            VIDEO_DECODER_NOT_AVAILABLE_NOW => VTError::VideoDecoderNotAvailableNow,
            PIXEL_ROTATION_NOT_SUPPORTED => VTError::PixelRotationNotSupported,
            VIDEO_ENCODER_NOT_AVAILABLE_NOW => VTError::VideoEncoderNotAvailableNow,
            FORMAT_DESCRIPTION_CHANGE_NOT_SUPPORTED => VTError::FormatDescriptionChangeNotSupported,
            INSUFFICIENT_SOURCE_COLOR_DATA => VTError::InsufficientSourceColorData,
            COULD_NOT_CREATE_COLOR_CORRECTION_DATA => VTError::CouldNotCreateColorCorrectionData,
            COLOR_SYNC_TRANSFORM_CONVERT_FAILED => VTError::ColorSyncTransformConvertFailed,
            VIDEO_DECODER_AUTHORIZATION => VTError::VideoDecoderAuthorization,
            VIDEO_ENCODER_AUTHORIZATION => VTError::VideoEncoderAuthorization,
            COLOR_CORRECTION_PIXEL_TRANSFER_FAILED => VTError::ColorCorrectionPixelTransferFailed,
            MULTI_PASS_STORAGE_IDENTIFIER_MISMATCH => VTError::MultiPassStorageIdentifierMismatch,
            MULTI_PASS_STORAGE_INVALID => VTError::MultiPassStorageInvalid,
            FRAME_SILO_INVALID_TIME_STAMP => VTError::FrameSiloInvalidTimeStamp,
            FRAME_SILO_INVALID_TIME_RANGE => VTError::FrameSiloInvalidTimeRange,
            COULD_NOT_FIND_TEMPORAL_FILTER => VTError::CouldNotFindTemporalFilter,
            PIXEL_TRANSFER_NOT_PERMITTED => VTError::PixelTransferNotPermitted,
            COLOR_CORRECTION_IMAGE_ROTATION_FAILED => VTError::ColorCorrectionImageRotationFailed,
            VIDEO_DECODER_REMOVED => VTError::VideoDecoderRemoved,
            SESSION_MALFUNCTION => VTError::SessionMalfunction,
            VIDEO_DECODER_NEEDS_ROSETTA => VTError::VideoDecoderNeedsRosetta,
            VIDEO_ENCODER_NEEDS_ROSETTA => VTError::VideoEncoderNeedsRosetta,
            VIDEO_DECODER_REFERENCE_MISSING => VTError::VideoDecoderReferenceMissing,
            VIDEO_DECODER_CALLBACK_MESSAGING => VTError::VideoDecoderCallbackMessaging,
            VIDEO_DECODER_UNKNOWN => VTError::VideoDecoderUnknown,
            EXTENSION_DISABLED => VTError::ExtensionDisabled,
            VIDEO_ENCODER_MVHEVCVIDEO_LAYER_IDS_MISMATCH => {
                VTError::VideoEncoderMVHEVCVideoLayerIdsMismatch
            }
            COULD_NOT_OUTPUT_TAGGED_BUFFER_GROUP => VTError::CouldNotOutputTaggedBufferGroup,
            COULD_NOT_FIND_EXTENSION => VTError::CouldNotFindExtension,
            EXTENSION_CONFLICT => VTError::ExtensionConflict,

            _ => VTError::UnknownError(value),
        }
    }
}
impl From<VTError> for OSStatus {
    fn from(value: VTError) -> Self {
        match value {
            VTError::PropertyNotSupported => PROPERTY_NOT_SUPPORTED,
            VTError::PropertyReadOnly => PROPERTY_READ_ONLY,
            VTError::Parameter => PARAMETER,
            VTError::InvalidSession => INVALID_SESSION,
            VTError::AllocationFailed => ALLOCATION_FAILED,
            VTError::PixelTransferNotSupported => PIXEL_TRANSFER_NOT_SUPPORTED,
            VTError::CouldNotFindVideoDecoder => COULD_NOT_FIND_VIDEO_DECODER,
            VTError::CouldNotCreateInstance => COULD_NOT_CREATE_INSTANCE,
            VTError::CouldNotFindVideoEncoder => COULD_NOT_FIND_VIDEO_ENCODER,
            VTError::VideoDecoderBadData => VIDEO_DECODER_BAD_DATA,
            VTError::VideoDecoderUnsupportedDataFormat => VIDEO_DECODER_UNSUPPORTED_DATA_FORMAT,
            VTError::VideoDecoderMalfunction => VIDEO_DECODER_MALFUNCTION,
            VTError::VideoEncoderMalfunction => VIDEO_ENCODER_MALFUNCTION,
            VTError::VideoDecoderNotAvailableNow => VIDEO_DECODER_NOT_AVAILABLE_NOW,
            VTError::ImageRotationNotSupported => PIXEL_ROTATION_NOT_SUPPORTED,
            VTError::PixelRotationNotSupported => PIXEL_ROTATION_NOT_SUPPORTED,
            VTError::VideoEncoderNotAvailableNow => VIDEO_ENCODER_NOT_AVAILABLE_NOW,
            VTError::FormatDescriptionChangeNotSupported => FORMAT_DESCRIPTION_CHANGE_NOT_SUPPORTED,
            VTError::InsufficientSourceColorData => INSUFFICIENT_SOURCE_COLOR_DATA,
            VTError::CouldNotCreateColorCorrectionData => COULD_NOT_CREATE_COLOR_CORRECTION_DATA,
            VTError::ColorSyncTransformConvertFailed => COLOR_SYNC_TRANSFORM_CONVERT_FAILED,
            VTError::VideoDecoderAuthorization => VIDEO_DECODER_AUTHORIZATION,
            VTError::VideoEncoderAuthorization => VIDEO_ENCODER_AUTHORIZATION,
            VTError::ColorCorrectionPixelTransferFailed => COLOR_CORRECTION_PIXEL_TRANSFER_FAILED,
            VTError::MultiPassStorageIdentifierMismatch => MULTI_PASS_STORAGE_IDENTIFIER_MISMATCH,
            VTError::MultiPassStorageInvalid => MULTI_PASS_STORAGE_INVALID,
            VTError::FrameSiloInvalidTimeStamp => FRAME_SILO_INVALID_TIME_STAMP,
            VTError::FrameSiloInvalidTimeRange => FRAME_SILO_INVALID_TIME_RANGE,
            VTError::CouldNotFindTemporalFilter => COULD_NOT_FIND_TEMPORAL_FILTER,
            VTError::PixelTransferNotPermitted => PIXEL_TRANSFER_NOT_PERMITTED,
            VTError::ColorCorrectionImageRotationFailed => COLOR_CORRECTION_IMAGE_ROTATION_FAILED,
            VTError::VideoDecoderRemoved => VIDEO_DECODER_REMOVED,
            VTError::SessionMalfunction => SESSION_MALFUNCTION,
            VTError::VideoDecoderNeedsRosetta => VIDEO_DECODER_NEEDS_ROSETTA,
            VTError::VideoEncoderNeedsRosetta => VIDEO_ENCODER_NEEDS_ROSETTA,
            VTError::VideoDecoderReferenceMissing => VIDEO_DECODER_REFERENCE_MISSING,
            VTError::VideoDecoderCallbackMessaging => VIDEO_DECODER_CALLBACK_MESSAGING,
            VTError::VideoDecoderUnknown => VIDEO_DECODER_UNKNOWN,
            VTError::ExtensionDisabled => EXTENSION_DISABLED,
            VTError::VideoEncoderMVHEVCVideoLayerIdsMismatch => {
                VIDEO_ENCODER_MVHEVCVIDEO_LAYER_IDS_MISMATCH
            }
            VTError::CouldNotOutputTaggedBufferGroup => COULD_NOT_OUTPUT_TAGGED_BUFFER_GROUP,
            VTError::CouldNotFindExtension => COULD_NOT_FIND_EXTENSION,
            VTError::ExtensionConflict => EXTENSION_CONFLICT,
            VTError::UnknownError(value) => value,
        }
    }
}
