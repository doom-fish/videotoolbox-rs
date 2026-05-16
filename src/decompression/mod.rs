//! [`DecompressionSession`] — hardware H.264/HEVC/ProRes decoder.
//!
//! Mirror of [`crate::compression::CompressionSession`] for the decoder
//! direction. Feed `CMSampleBuffer`s in via [`DecompressionSession::decode`],
//! receive `CVPixelBuffer`s out via the callback you registered at construction.

use core::ffi::c_void;
use core::ptr;
use std::sync::{Arc, Mutex};

use crate::error::VTError;
use crate::ffi;

/// One decoded video frame.
pub struct DecodedFrame {
    /// Underlying `CVImageBuffer` (typically a `CVPixelBuffer`) — `None`
    /// when the decoder dropped or skipped a frame.
    pub image_buffer: Option<apple_cf::cv::CVPixelBuffer>,
    /// Presentation timestamp of the source sample buffer.
    pub presentation_time: (i64, i32),
    /// Presentation duration of the decoded frame.
    pub duration: (i64, i32),
    /// Decoder hint flags (asynchronous / image-buffer-modifiable / etc.).
    pub info_flags: u32,
    /// Apple's decoder status code. 0 indicates success.
    pub status: i32,
}

type DecodeCallback = Box<dyn FnMut(DecodedFrame) + Send + 'static>;

struct CallbackState {
    callback: Mutex<DecodeCallback>,
}

/// Hardware video decompression session.
///
/// Build with [`DecompressionSession::new`] passing a format description
/// (from the encoder side, or extracted from your input file). Decoded
/// frames flow through the callback you supply.
///
/// # Lifetime / threading
///
/// Drop the session to invalidate it. Decoder callbacks fire on Apple's
/// internal queue — they may run after `decode()` returns and before
/// `wait_for_async_frames()`.
pub struct DecompressionSession {
    session: ffi::VTDecompressionSessionRef,
    _state: Arc<CallbackState>,
}

unsafe impl Send for DecompressionSession {}
unsafe impl Sync for DecompressionSession {}

impl Drop for DecompressionSession {
    fn drop(&mut self) {
        if !self.session.is_null() {
            unsafe {
                ffi::VTDecompressionSessionInvalidate(self.session);
                ffi::CFRelease(self.session.cast_const());
            }
            self.session = ptr::null_mut();
        }
    }
}

impl DecompressionSession {
    /// Open a decompression session for the given format description.
    /// `format_description` is the `CMFormatDescriptionRef` Apple expects
    /// — typically obtained from `CMSampleBuffer.format_description()` on
    /// the first sample of the stream.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncoderCallback`] wrapping the raw `OSStatus`
    /// if `VTDecompressionSessionCreate` fails.
    pub fn new<F>(
        format_description: &apple_cf::cm::CMFormatDescription,
        callback: F,
    ) -> Result<Self, VTError>
    where
        F: FnMut(DecodedFrame) + Send + 'static,
    {
        let state = Arc::new(CallbackState {
            callback: Mutex::new(Box::new(callback)),
        });
        let state_for_callback = state.clone();
        let ref_con = Arc::into_raw(state_for_callback).cast::<c_void>().cast_mut();

        let record = ffi::VTDecompressionOutputCallbackRecord {
            decompression_output_callback: decode_trampoline,
            decompression_output_ref_con: ref_con,
        };

        let mut session: ffi::VTDecompressionSessionRef = ptr::null_mut();
        let status = unsafe {
            ffi::VTDecompressionSessionCreate(
                ffi::kCFAllocatorDefault,
                format_description.as_ptr().cast(),
                ptr::null(),
                ptr::null(),
                &record,
                &mut session,
            )
        };
        if status != 0 {
            // Recover the Arc so it doesn't leak.
            unsafe { Arc::from_raw(ref_con.cast::<CallbackState>()) };
            return Err(VTError::EncoderCallback(status));
        }
        Ok(Self {
            session,
            _state: state,
        })
    }

    /// Decode `sample_buffer`. The callback you registered at construction
    /// will fire with the resulting [`DecodedFrame`] — either synchronously
    /// before this method returns, or asynchronously after, depending on
    /// the decoder backend.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncoderCallback`] wrapping the raw `OSStatus`
    /// if `VTDecompressionSessionDecodeFrame` rejects the sample buffer.
    pub fn decode(
        &self,
        sample_buffer: &apple_cf::cm::CMSampleBuffer,
    ) -> Result<(), VTError> {
        let mut info_flags: u32 = 0;
        let status = unsafe {
            ffi::VTDecompressionSessionDecodeFrame(
                self.session,
                sample_buffer.as_ptr(),
                0,
                ptr::null_mut(),
                &mut info_flags,
            )
        };
        if status == 0 {
            Ok(())
        } else {
            Err(VTError::EncoderCallback(status))
        }
    }

    /// Wait for any queued async decodes to complete. Equivalent to
    /// `VTDecompressionSessionWaitForAsynchronousFrames`. Call this
    /// before dropping the session if you want to be sure every frame's
    /// callback has fired.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncoderCallback`] on a non-zero `OSStatus`.
    pub fn wait_for_async_frames(&self) -> Result<(), VTError> {
        let status =
            unsafe { ffi::VTDecompressionSessionWaitForAsynchronousFrames(self.session) };
        if status == 0 {
            Ok(())
        } else {
            Err(VTError::EncoderCallback(status))
        }
    }

    /// Set an arbitrary property on the underlying
    /// `VTDecompressionSession`. `value` must be a CoreFoundation
    /// object matching Apple's per-key contract (see
    /// `VTDecompressionProperties.h`).
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SetPropertyFailed`] if Apple rejects the
    /// key/value pair.
    ///
    /// # Safety
    ///
    /// `key` must be a valid `CFStringRef` and `value` must be a
    /// valid CoreFoundation pointer for the property's expected type.
    pub unsafe fn set_property(
        &self,
        key: ffi::CFStringRef,
        value: ffi::CFTypeRef,
    ) -> Result<(), VTError> {
        let status = ffi::VTSessionSetProperty(self.session.cast(), key, value);
        if status != 0 {
            return Err(VTError::SetPropertyFailed {
                key: "<custom>".to_string(),
                status,
            });
        }
        Ok(())
    }

    /// Tell the decoder that this is real-time playback (prioritise
    /// low latency over throughput). Wraps the boolean property
    /// `kVTDecompressionPropertyKey_RealTime`.
    ///
    /// # Errors
    ///
    /// See [`Self::set_property`].
    pub fn set_real_time(&self, real_time: bool) -> Result<(), VTError> {
        let v = unsafe {
            if real_time {
                ffi::kCFBooleanTrue
            } else {
                ffi::kCFBooleanFalse
            }
        };
        unsafe {
            self.set_property(ffi::kVTDecompressionPropertyKey_RealTime, v.cast())
        }
    }

    /// Suggest a maximum number of frames the decoder may keep
    /// internally awaiting reordering (B-frame reorder depth).
    /// Wraps `kVTDecompressionPropertyKey_MaximumOutputBufferDepth`.
    ///
    /// # Errors
    ///
    /// See [`Self::set_property`].
    pub fn set_max_output_buffer_depth(&self, depth: i32) -> Result<(), VTError> {
        let v = unsafe {
            ffi::CFNumberCreate(
                ffi::kCFAllocatorDefault,
                ffi::kCFNumberSInt32Type,
                core::ptr::from_ref(&depth).cast(),
            )
        };
        let r = unsafe {
            self.set_property(
                ffi::kVTDecompressionPropertyKey_MaximumOutputBufferDepth,
                v.cast(),
            )
        };
        unsafe { ffi::CFRelease(v) };
        r
    }

    /// Finish any decoded frames the decoder has been holding back
    /// for B-frame reordering. Wraps
    /// `VTDecompressionSessionFinishDelayedFrames`.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncoderCallback`] on non-zero `OSStatus`.
    pub fn finish_delayed_frames(&self) -> Result<(), VTError> {
        let status =
            unsafe { ffi::VTDecompressionSessionFinishDelayedFrames(self.session) };
        if status == 0 {
            Ok(())
        } else {
            Err(VTError::EncoderCallback(status))
        }
    }

    /// True if this session can decode samples described by `format`
    /// without rebuilding the session. Wraps
    /// `VTDecompressionSessionCanAcceptFormatDescription`.
    ///
    /// # Safety
    ///
    /// `format` must be a valid `CMFormatDescriptionRef` (typically
    /// obtained from another `CMSampleBuffer`).
    pub unsafe fn can_accept_format(&self, format: ffi::CMFormatDescriptionRef) -> bool {
        ffi::VTDecompressionSessionCanAcceptFormatDescription(self.session, format)
    }
}

unsafe extern "C" fn decode_trampoline(
    output_ref_con: *mut c_void,
    _source_frame_ref_con: *mut c_void,
    status: ffi::OSStatus,
    info_flags: u32,
    image_buffer: *mut c_void,
    pts: ffi::CMTime,
    duration: ffi::CMTime,
) {
    if output_ref_con.is_null() {
        return;
    }
    // Borrow the Arc without consuming the reference held by the session.
    let state = unsafe { Arc::from_raw(output_ref_con.cast::<CallbackState>()) };
    let state_clone = state.clone();
    core::mem::forget(state);

    let image = if image_buffer.is_null() {
        None
    } else {
        // The decoder owns the pixel buffer; retain so the Rust wrapper
        // can extend its lifetime past the callback's stack frame.
        unsafe { ffi::CFRetain(image_buffer.cast_const()) };
        apple_cf::cv::CVPixelBuffer::from_raw(image_buffer)
    };

    let frame = DecodedFrame {
        image_buffer: image,
        presentation_time: (pts.value, pts.timescale),
        duration: (duration.value, duration.timescale),
        info_flags,
        status,
    };

    let Ok(mut guard) = state_clone.callback.lock() else {
        return;
    };
    guard(frame);
}
