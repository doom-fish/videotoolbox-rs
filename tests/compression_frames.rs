#![cfg(feature = "compression")]

mod common;

use apple_cf::cf::{AsCFType, CFDictionary, CFNumber, CFString};
use apple_cf::cm::CMTime;
use common::make_test_surface;
use videotoolbox::{
    available_video_encoders, ffi, Codec, CompressionSession, FrameProperties,
    HardwareAcceleration, ProfileLevel, VTError,
};

fn reordering_session() -> Result<CompressionSession, VTError> {
    CompressionSession::builder(64, 64, Codec::H264)
        .with_real_time(false)
        .with_allow_frame_reordering(true)
        .with_profile_level(ProfileLevel::H264HighAutoLevel)
        .with_expected_frame_rate(30.0)
        .with_max_keyframe_interval(30)
        .build()
}

fn is_true(value: Option<apple_cf::cf::CFType>) -> bool {
    value.is_some_and(|value| value.as_ptr().cast_const() == unsafe { ffi::kCFBooleanTrue }.cast())
}

fn nal_units(data: &[u8], length_size: usize) -> Vec<&[u8]> {
    let mut units = Vec::new();
    let mut offset = 0;
    while offset < data.len() {
        let header = &data[offset..offset + length_size];
        let length = header
            .iter()
            .fold(0_usize, |length, byte| (length << 8) | usize::from(*byte));
        offset += length_size;
        units.push(&data[offset..offset + length]);
        offset += length;
    }
    assert_eq!(offset, data.len(), "length prefixes must tile the frame");
    units
}

#[test]
fn sync_encode_with_reordering_returns_each_frame_for_its_own_timestamp() -> Result<(), VTError> {
    let session = reordering_session()?;
    let surface = make_test_surface(64, 64);

    for index in 0..12 {
        let presentation_time = CMTime::new(index, 30);
        let frame = session.encode(&surface, presentation_time)?;
        assert_eq!(frame.presentation_time, presentation_time);
        assert!(!frame.data.is_empty());
    }
    Ok(())
}

#[test]
fn forced_keyframes_become_sync_samples() -> Result<(), VTError> {
    let session = CompressionSession::builder(64, 64, Codec::H264)
        .with_real_time(true)
        .with_allow_frame_reordering(false)
        .with_expected_frame_rate(30.0)
        .with_max_keyframe_interval(1000)
        .build()?;
    let surface = make_test_surface(64, 64);
    let is_sync = |frame: &videotoolbox::EncodedFrame| {
        frame
            .cm_sample_buffer()
            .expect("frame was not dropped")
            .is_sync_sample()
    };

    let first = session.encode(&surface, CMTime::new(0, 30))?;
    let second = session.encode(&surface, CMTime::new(1, 30))?;
    let forced = session.encode_with_properties(
        &surface,
        CMTime::new(2, 30),
        FrameProperties::new().with_force_key_frame(true),
    )?;
    let after =
        session.encode_with_properties(&surface, CMTime::new(3, 30), FrameProperties::new())?;

    assert!(is_sync(&first));
    assert!(!is_sync(&second));
    assert!(is_sync(&forced));
    assert!(!is_sync(&after));
    Ok(())
}

#[test]
fn h264_frames_are_length_prefixed_and_parameter_sets_live_in_the_format() -> Result<(), VTError> {
    let frame = common::encode_h264_test_frame(64, 64)?;
    let format = frame
        .cm_sample_buffer()
        .and_then(apple_cf::cm::CMSampleBuffer::format_description)
        .expect("encoded frame has a format description");

    let parameter_sets = format
        .video_parameter_sets()
        .expect("H.264 format carries parameter sets");
    let nal_types: Vec<u8> = parameter_sets
        .parameter_sets
        .iter()
        .map(|set| set[0] & 0x1f)
        .collect();
    assert_eq!(nal_types, [7, 8]);

    let length_size = usize::try_from(parameter_sets.nal_unit_header_length).expect("positive");
    let units = nal_units(&frame.data, length_size);
    assert!(
        units.iter().any(|unit| unit[0] & 0x1f == 5),
        "keyframe carries an IDR slice"
    );
    assert!(units
        .iter()
        .all(|unit| unit[0] & 0x1f != 7 && unit[0] & 0x1f != 8));
    Ok(())
}

#[test]
fn hevc_parameter_sets_come_from_the_format_description() -> Result<(), VTError> {
    let session = match CompressionSession::builder(128, 128, Codec::HEVC)
        .with_real_time(true)
        .build()
    {
        Ok(session) => session,
        Err(VTError::SessionCreateFailed(_)) => return Ok(()),
        Err(error) => return Err(error),
    };
    let frame = session.encode(&make_test_surface(128, 128), CMTime::new(0, 30))?;
    let format = frame
        .cm_sample_buffer()
        .and_then(apple_cf::cm::CMSampleBuffer::format_description)
        .expect("encoded frame has a format description");

    let parameter_sets = format
        .video_parameter_sets()
        .expect("HEVC format carries parameter sets");
    let nal_types: Vec<u8> = parameter_sets
        .parameter_sets
        .iter()
        .map(|set| (set[0] >> 1) & 0x3f)
        .collect();
    assert!(
        nal_types.starts_with(&[32, 33, 34]),
        "VPS, SPS, PPS: {nal_types:?}"
    );

    let length_size = usize::try_from(parameter_sets.nal_unit_header_length).expect("positive");
    assert!(!nal_units(&frame.data, length_size).is_empty());
    Ok(())
}

fn encoder_id(session: &CompressionSession) -> Result<String, VTError> {
    let id = unsafe { session.copy_property(ffi::kVTCompressionPropertyKey_EncoderID) }?
        .expect("encoder reports its ID");
    Ok(unsafe { CFString::from_raw_borrowed(id.as_ptr()) }
        .expect("string")
        .to_string())
}

#[test]
fn hardware_acceleration_choice_reaches_the_encoder() -> Result<(), VTError> {
    let using_hardware =
        unsafe { ffi::kVTCompressionPropertyKey_UsingHardwareAcceleratedVideoEncoder };

    let software = CompressionSession::builder(256, 256, Codec::H264)
        .with_hardware_acceleration(HardwareAcceleration::Disabled)
        .build()?;
    match unsafe { software.copy_property(using_hardware) } {
        Ok(value) => assert!(!is_true(value)),
        Err(VTError::ApiFailed { status: -12900, .. }) => {}
        Err(error) => return Err(error),
    }

    match CompressionSession::builder(256, 256, Codec::H264)
        .with_hardware_acceleration(HardwareAcceleration::Required)
        .build()
    {
        Ok(hardware) => {
            assert!(is_true(unsafe { hardware.copy_property(using_hardware) }?));
            assert_ne!(encoder_id(&hardware)?, encoder_id(&software)?);
        }
        Err(VTError::SessionCreateFailed(_)) => {}
        Err(error) => return Err(error),
    }
    Ok(())
}

#[test]
fn encoder_id_selects_that_encoder() -> Result<(), VTError> {
    let encoders = available_video_encoders().map_err(|status| VTError::ApiFailed {
        api: "VTCopyVideoEncoderList",
        status,
    })?;
    let Some(encoder) = encoders
        .iter()
        .find(|encoder| encoder.codec_type == Codec::H264.as_cm_codec_type())
    else {
        return Ok(());
    };

    let session = CompressionSession::builder(256, 256, Codec::H264)
        .with_encoder_id(encoder.encoder_id.clone())
        .build()?;
    assert_eq!(encoder_id(&session)?, encoder.encoder_id);

    let unknown = CompressionSession::builder(256, 256, Codec::H264)
        .with_encoder_id("com.example.no-such-encoder")
        .build();
    assert!(matches!(unknown, Err(VTError::SessionCreateFailed(_))));
    Ok(())
}

#[test]
fn low_latency_rate_control_session_encodes() -> Result<(), VTError> {
    let session = match CompressionSession::builder(256, 256, Codec::H264)
        .with_low_latency_rate_control(true)
        .with_real_time(true)
        .with_average_bit_rate(1_000_000)
        .build()
    {
        Ok(session) => session,
        Err(VTError::SessionCreateFailed(_)) => return Ok(()),
        Err(error) => return Err(error),
    };
    let frame = session.encode(&make_test_surface(256, 256), CMTime::new(0, 30))?;
    assert!(!frame.data.is_empty());
    Ok(())
}

#[test]
fn source_pixel_buffer_attributes_shape_the_input_pool() -> Result<(), VTError> {
    const NV12: i64 = 0x3432_3076;
    let pixel_format_key = CFString::new("PixelFormatType");
    let pixel_format = CFNumber::from_i64(NV12);
    let attributes = CFDictionary::from_pairs(&[(
        &pixel_format_key as &dyn AsCFType,
        &pixel_format as &dyn AsCFType,
    )]);

    let session = CompressionSession::builder(64, 64, Codec::H264)
        .with_source_pixel_buffer_attributes(attributes)
        .build()?;
    let pool = session
        .pixel_buffer_pool()
        .expect("session has an input pool");
    let pixel_buffer = pool
        .create_pixel_buffer()
        .map_err(|status| VTError::ApiFailed {
            api: "CVPixelBufferPoolCreatePixelBuffer",
            status,
        })?;

    assert_eq!(i64::from(pixel_buffer.pixel_format()), NV12);
    assert_eq!(pixel_buffer.width(), 64);
    Ok(())
}

#[cfg(feature = "async")]
mod asynchronous {
    use apple_cf::cm::CMTime;
    use videotoolbox::VTError;

    use apple_cf::cv::CVPixelBuffer;

    use super::common::{
        fill_bgra_pixels, make_bgra_pixel_buffer, make_test_pixel_buffer, make_test_surface,
    };
    use super::reordering_session;

    fn moving_frame(index: usize) -> CVPixelBuffer {
        let buffer = make_bgra_pixel_buffer(64, 64);
        let pixels: Vec<[u8; 4]> = (0..64 * 64)
            .map(|pixel| {
                let shade =
                    |step: usize| u8::try_from((pixel * step + index * 31) % 251).expect("< 251");
                [shade(7), shade(1), shade(3), 0xFF]
            })
            .collect();
        fill_bgra_pixels(&buffer, &pixels);
        buffer
    }

    #[test]
    fn sequential_awaits_resolve_with_frame_reordering_enabled() -> Result<(), VTError> {
        let session = reordering_session()?;
        pollster::block_on(async {
            for index in 0..12 {
                let presentation_time = CMTime::new(index, 30);
                let sample = session
                    .encode_frame_async(
                        make_test_pixel_buffer(64, 64),
                        presentation_time,
                        CMTime::new(1, 30),
                        None,
                    )
                    .await?;
                assert_eq!(sample.presentation_timestamp(), presentation_time);
            }
            Ok(())
        })
    }

    #[test]
    fn pipelined_frames_resolve_to_their_own_source_in_any_await_order() -> Result<(), VTError> {
        let session = reordering_session()?;
        let mut pending: Vec<_> = (0..12)
            .map(|index| {
                (
                    CMTime::new(index, 30),
                    session.encode_frame_async(
                        moving_frame(usize::try_from(index).expect("small")),
                        CMTime::new(index, 30),
                        CMTime::new(1, 30),
                        None,
                    ),
                )
            })
            .collect();
        pending.reverse();

        pollster::block_on(async {
            for (presentation_time, frame) in pending {
                assert_eq!(frame.await?.presentation_timestamp(), presentation_time);
            }
            Ok(())
        })
    }

    #[test]
    fn sync_encode_never_receives_a_pending_async_frame() -> Result<(), VTError> {
        let session = reordering_session()?;
        let pending: Vec<_> = (0..4)
            .map(|index| {
                (
                    CMTime::new(index, 30),
                    session.encode_frame_async(
                        moving_frame(usize::try_from(index).expect("small")),
                        CMTime::new(index, 30),
                        CMTime::new(1, 30),
                        None,
                    ),
                )
            })
            .collect();

        let sync_time = CMTime::new(4, 30);
        let sync_frame = session.encode(&make_test_surface(64, 64), sync_time)?;
        assert_eq!(sync_frame.presentation_time, sync_time);

        pollster::block_on(async {
            for (presentation_time, frame) in pending {
                assert_eq!(frame.await?.presentation_timestamp(), presentation_time);
            }
            Ok(())
        })
    }

    #[test]
    fn dropping_an_unawaited_frame_does_not_shift_later_frames() -> Result<(), VTError> {
        let session = reordering_session()?;
        for index in 0..3 {
            drop(session.encode_frame_async(
                make_test_pixel_buffer(64, 64),
                CMTime::new(index, 30),
                CMTime::new(1, 30),
                None,
            ));
        }

        for index in 3..6 {
            let presentation_time = CMTime::new(index, 30);
            let frame = session.encode(&make_test_surface(64, 64), presentation_time)?;
            assert_eq!(frame.presentation_time, presentation_time);
        }
        session.invalidate()
    }
}
