#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]

//! `VTRAWProcessingSession` — `ProRes` RAW / `CinemaDNG` decoder with
//! per-frame parameter controls (macOS 15+).
//!
//! Session creation / parameter readback / parameter writeback /
//! complete-frames all go through the pure-C FFI. The Swift bridge wraps
//! the async `process(frame:)` API for both blocking and future-based Rust
//! entry points.

use core::ffi::c_void;
use core::ptr;
use std::sync::{Arc, Condvar, Mutex};

use apple_cf::{cf::CFType, cm::CMFormatDescription, cv::CVPixelBuffer};
#[cfg(feature = "async")]
use doom_fish_utils::completion::{AsyncCompletion, SyncCompletionPtr};
#[cfg(feature = "async")]
use doom_fish_utils::panic_safe::catch_user_panic;

use crate::error::VTError;
use crate::ffi;
use crate::session;

#[cfg(feature = "async")]
type RawProcessFrameAsyncCallback = unsafe extern "C" fn(*mut c_void, i32, *mut c_void);
type RawParameterContextRelease = unsafe extern "C" fn(*mut c_void);

extern "C" {
    fn vtb_raw_session_process_frame(
        session: *mut c_void,
        input_pixel_buffer: *mut c_void,
        out: *mut *mut c_void,
    ) -> i32;
    #[cfg(feature = "async")]
    fn vtb_raw_session_process_frame_async(
        session: *mut c_void,
        input_pixel_buffer: *mut c_void,
        refcon: *mut c_void,
        callback: Option<RawProcessFrameAsyncCallback>,
    ) -> i32;
    fn vtb_raw_session_set_parameter_changed_handler(
        session: *mut c_void,
        refcon: *mut c_void,
        callback: Option<unsafe extern "C" fn(*mut c_void, ffi::CFArrayRef)>,
        context_release: Option<RawParameterContextRelease>,
    ) -> i32;
}

type ParameterChangedCallback = Box<dyn FnMut(Vec<RawProcessingParameter>) + Send + 'static>;
const VTB_TIMED_OUT: i32 = i32::from_be_bytes(*b"vtto");

struct ParameterChangedContext {
    invocation: Mutex<ParameterChangedInvocation>,
    available: Condvar,
}

struct ParameterChangedInvocation {
    callback: Option<ParameterChangedCallback>,
    owner: Option<std::thread::ThreadId>,
}

impl ParameterChangedContext {
    fn new(callback: ParameterChangedCallback) -> Self {
        Self {
            invocation: Mutex::new(ParameterChangedInvocation {
                callback: Some(callback),
                owner: None,
            }),
            available: Condvar::new(),
        }
    }

    fn invoke(&self, parameters: Vec<RawProcessingParameter>) {
        let current_thread = std::thread::current().id();
        let mut invocation = self
            .invocation
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            if invocation.callback.is_some() {
                break;
            }
            if invocation.owner.as_ref() == Some(&current_thread) {
                return;
            }
            invocation = self
                .available
                .wait(invocation)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }

        let mut callback = invocation
            .callback
            .take()
            .expect("parameter callback must be available");
        invocation.owner = Some(current_thread);
        drop(invocation);

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            callback(parameters);
        }));

        let mut invocation = self
            .invocation
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        invocation.owner = None;
        invocation.callback = Some(callback);
        drop(invocation);
        self.available.notify_one();
    }
}

struct ParameterChangedHandlerState {
    current: Mutex<Option<Arc<ParameterChangedContext>>>,
}

impl ParameterChangedHandlerState {
    fn transition<F>(&self, next: Option<Arc<ParameterChangedContext>>, update_native: F) -> i32
    where
        F: FnOnce() -> i32,
    {
        let mut current = self
            .current
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let status = update_native();
        let retired = if status == 0 {
            Some(core::mem::replace(&mut *current, next))
        } else {
            None
        };
        drop(current);
        drop(retired);
        status
    }
}

#[cfg(feature = "async")]
struct AsyncRawProcessingContext(SyncCompletionPtr);

/// `VTRAWProcessingSessionRef`.
pub struct RawProcessingSession {
    inner: ffi::VTRAWProcessingSessionRef,
    parameter_changed_handler: Arc<ParameterChangedHandlerState>,
}

unsafe impl Send for RawProcessingSession {}
unsafe impl Sync for RawProcessingSession {}

impl Drop for RawProcessingSession {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            let _ = self.parameter_changed_handler.transition(None, || unsafe {
                vtb_raw_session_set_parameter_changed_handler(
                    self.inner.cast(),
                    ptr::null_mut(),
                    None,
                    None,
                )
            });
            unsafe {
                ffi::VTRAWProcessingSessionInvalidate(self.inner);
                ffi::CFRelease(self.inner.cast());
            }
            self.inner = ptr::null_mut();
        }
    }
}

impl RawProcessingSession {
    /// CoreFoundation type identifier for `VTRAWProcessingSession`.
    #[must_use]
    pub fn type_id() -> usize {
        unsafe { ffi::VTRAWProcessingSessionGetTypeID() }
    }

    /// Create a RAW processing session for the given video format.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`] on failure.
    pub fn new(format: &CMFormatDescription) -> Result<Self, VTError> {
        let mut p: ffi::VTRAWProcessingSessionRef = ptr::null_mut();
        let s = unsafe {
            ffi::VTRAWProcessingSessionCreate(
                ffi::kCFAllocatorDefault,
                format.as_ptr().cast::<c_void>(),
                ptr::null(),
                ptr::null(),
                &mut p,
            )
        };
        if s != 0 || p.is_null() {
            return Err(VTError::SessionCreateFailed(s));
        }
        Ok(Self {
            inner: p,
            parameter_changed_handler: Arc::new(ParameterChangedHandlerState {
                current: Mutex::new(None),
            }),
        })
    }

    /// Force-complete any outstanding frames.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn complete_frames(&self) -> Result<(), VTError> {
        let s = unsafe { ffi::VTRAWProcessingSessionCompleteFrames(self.inner) };
        if s == 0 {
            Ok(())
        } else {
            Err(VTError::EncodeFailed(s))
        }
    }

    /// Copy a raw `VTSession` property from the underlying RAW processing session.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] if `VTSessionCopyProperty` fails.
    ///
    /// # Safety
    ///
    /// `key` must be a valid `CFStringRef` exported by `VideoToolbox`.
    pub unsafe fn copy_property(&self, key: ffi::CFStringRef) -> Result<Option<CFType>, VTError> {
        unsafe { session::copy_property(self.inner.cast(), key) }
    }

    /// Copy the sidecar-file metadata blob, when the active RAW processor exposes it.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] if `VTSessionCopyProperty` fails.
    pub fn metadata_for_sidecar_file(&self) -> Result<Option<CFType>, VTError> {
        unsafe { self.copy_property(ffi::kVTRAWProcessingPropertyKey_MetadataForSidecarFile) }
    }

    /// Copy the requested Metal-device registry ID, when the active RAW processor exposes it.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] if `VTSessionCopyProperty` fails.
    pub fn metal_device_registry_id(&self) -> Result<Option<CFType>, VTError> {
        unsafe { self.copy_property(ffi::kVTRAWProcessingPropertyKey_MetalDeviceRegistryID) }
    }

    /// Copy the output color-attachment dictionary, when the active RAW processor exposes it.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] if `VTSessionCopyProperty` fails.
    pub fn output_color_attachments(&self) -> Result<Option<CFType>, VTError> {
        unsafe { self.copy_property(ffi::kVTRAWProcessingPropertyKey_OutputColorAttachments) }
    }

    /// Copy the array of processing parameters this RAW codec
    /// exposes.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    pub fn parameters(&self) -> Result<Vec<RawProcessingParameter>, VTError> {
        let mut arr: ffi::CFArrayRef = ptr::null();
        let s =
            unsafe { ffi::VTRAWProcessingSessionCopyProcessingParameters(self.inner, &mut arr) };
        if s != 0 || arr.is_null() {
            return Err(VTError::EncodeFailed(s));
        }
        let out = parameters_from_array(arr);
        unsafe { ffi::CFRelease(arr.cast()) };
        Ok(out)
    }

    /// Set processing parameters. The dictionary keys must match the
    /// `kVTRAWProcessingParameter_Key` strings returned by
    /// [`parameters`](Self::parameters); values must conform to the
    /// declared value type.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] on `OSStatus` failure.
    ///
    /// # Safety
    ///
    /// `params` must be a valid `CFDictionaryRef`.
    pub unsafe fn set_parameters_raw(&self, params: ffi::CFDictionaryRef) -> Result<(), VTError> {
        let s = unsafe { ffi::VTRAWProcessingSessionSetProcessingParameters(self.inner, params) };
        if s == 0 {
            Ok(())
        } else {
            Err(VTError::EncodeFailed(s))
        }
    }

    /// Install a callback that fires when the RAW processor changes its
    /// available parameters or their current values.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] when the framework rejects the handler.
    /// Concurrent installs and clears are serialized, and a callback may replace
    /// or clear itself without holding the handler-transition lock.
    pub fn set_parameter_changed_handler<F>(&self, callback: F) -> Result<(), VTError>
    where
        F: FnMut(Vec<RawProcessingParameter>) + Send + 'static,
    {
        let next = Arc::new(ParameterChangedContext::new(Box::new(callback)));
        let native_context = Arc::clone(&next);
        let status = self.parameter_changed_handler.transition(Some(next), || {
            let refcon = Arc::into_raw(native_context).cast_mut().cast();
            unsafe {
                vtb_raw_session_set_parameter_changed_handler(
                    self.inner.cast(),
                    refcon,
                    Some(raw_parameter_changed_trampoline),
                    Some(raw_parameter_changed_context_release),
                )
            }
        });
        if status != 0 {
            return Err(VTError::ApiFailed {
                api: "VTRAWProcessingSessionSetParameterChangedHandler",
                status,
            });
        }
        Ok(())
    }

    /// Remove any previously-installed parameter-change handler.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::ApiFailed`] when the framework rejects the update.
    pub fn clear_parameter_changed_handler(&self) -> Result<(), VTError> {
        let status = self.parameter_changed_handler.transition(None, || unsafe {
            vtb_raw_session_set_parameter_changed_handler(
                self.inner.cast(),
                ptr::null_mut(),
                None,
                None,
            )
        });
        if status != 0 {
            return Err(VTError::ApiFailed {
                api: "VTRAWProcessingSessionSetParameterChangedHandler",
                status,
            });
        }
        Ok(())
    }

    /// Process a single RAW input frame, returning the processed
    /// `CVPixelBuffer`. Blocks on Swift's `process(frame:)` async
    /// call internally.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::TimedOut`] if the Swift async operation exceeds its
    /// bounded wait, or [`VTError::EncodeFailed`] on another `OSStatus` failure.
    pub fn process(&self, input: &CVPixelBuffer) -> Result<CVPixelBuffer, VTError> {
        let mut out: *mut c_void = ptr::null_mut();
        let s = unsafe {
            vtb_raw_session_process_frame(self.inner, input.as_ptr().cast::<c_void>(), &mut out)
        };
        if s == VTB_TIMED_OUT {
            return Err(VTError::TimedOut {
                operation: "RawProcessingSession::process",
            });
        }
        if s != 0 || out.is_null() {
            return Err(VTError::EncodeFailed(s));
        }
        unsafe { CVPixelBuffer::from_raw(out.cast()) }.ok_or(VTError::EncodeFailed(0))
    }

    /// Submit `input_pixel_buffer` for processing and await the processed output.
    ///
    /// This method requires the crate's `async` feature.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::EncodeFailed`] if the RAW processor rejects the frame
    /// or completes without a processed `CVPixelBuffer`.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub async fn process_frame_async(
        &self,
        input_pixel_buffer: CVPixelBuffer,
    ) -> Result<CVPixelBuffer, VTError> {
        let (future, completion) = AsyncCompletion::<CVPixelBuffer>::create();
        let context = Box::into_raw(Box::new(AsyncRawProcessingContext(completion)));
        let status = unsafe {
            vtb_raw_session_process_frame_async(
                self.inner.cast(),
                input_pixel_buffer.as_ptr().cast::<c_void>(),
                context.cast::<c_void>(),
                Some(raw_process_async_trampoline),
            )
        };
        if status != 0 {
            let context = unsafe { Box::from_raw(context) };
            unsafe {
                AsyncCompletion::<CVPixelBuffer>::complete_err(context.0, status.to_string());
            };
            return Err(VTError::EncodeFailed(status));
        }

        future
            .await
            .map_err(|error| VTError::EncodeFailed(parse_async_status(&error)))
    }

    /// Raw `VTRAWProcessingSessionRef`.
    #[must_use]
    pub const fn as_ptr(&self) -> ffi::VTRAWProcessingSessionRef {
        self.inner
    }
}

fn parameters_from_array(arr: ffi::CFArrayRef) -> Vec<RawProcessingParameter> {
    if arr.is_null() {
        return Vec::new();
    }

    let count = unsafe { ffi::CFArrayGetCount(arr) };
    let mut out = Vec::with_capacity(count.max(0) as usize);
    for i in 0..count {
        let dict = unsafe { ffi::CFArrayGetValueAtIndex(arr, i) };
        if !dict.is_null() {
            unsafe { ffi::CFRetain(dict.cast()) };
            out.push(RawProcessingParameter { dict: dict.cast() });
        }
    }
    out
}

unsafe extern "C" fn raw_parameter_changed_trampoline(
    refcon: *mut c_void,
    parameter_array: ffi::CFArrayRef,
) {
    let Some(context) = (unsafe { refcon.cast::<ParameterChangedContext>().as_ref() }) else {
        return;
    };
    context.invoke(parameters_from_array(parameter_array));
}

unsafe extern "C" fn raw_parameter_changed_context_release(refcon: *mut c_void) {
    if refcon.is_null() {
        return;
    }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        unsafe { drop(Arc::from_raw(refcon.cast::<ParameterChangedContext>())) };
    }));
}

#[cfg(feature = "async")]
fn parse_async_status(error: &str) -> ffi::OSStatus {
    error.parse().unwrap_or(-1)
}

#[cfg(feature = "async")]
unsafe extern "C" fn raw_process_async_trampoline(
    refcon: *mut c_void,
    status: i32,
    output_pixel_buffer: *mut c_void,
) {
    catch_user_panic("videotoolbox::raw_processing::process_frame_async", || {
        let context = unsafe { Box::from_raw(refcon.cast::<AsyncRawProcessingContext>()) };
        if status != 0 {
            unsafe {
                AsyncCompletion::<CVPixelBuffer>::complete_err(context.0, status.to_string());
            };
            return;
        }

        let Some(pixel_buffer) = (unsafe { CVPixelBuffer::from_raw(output_pixel_buffer.cast()) })
        else {
            unsafe {
                AsyncCompletion::<CVPixelBuffer>::complete_err(context.0, "-1".into());
            };
            return;
        };

        unsafe {
            AsyncCompletion::<CVPixelBuffer>::complete_ok(context.0, pixel_buffer);
        };
    });
}

/// A single RAW-processing parameter descriptor.
///
/// Wraps a `CFDictionary` returned by `VTRAWProcessingSession::parameters`.
pub struct RawProcessingParameter {
    dict: ffi::CFDictionaryRef,
}

unsafe impl Send for RawProcessingParameter {}
unsafe impl Sync for RawProcessingParameter {}

crate::utils::retained::vt_retained!(
    RawProcessingParameter,
    field = dict,
    release = ffi::CFRelease
);

impl RawProcessingParameter {
    /// Raw `CFDictionaryRef`.
    #[must_use]
    pub const fn as_ptr(&self) -> ffi::CFDictionaryRef {
        self.dict
    }

    /// Stable identifier used as the key when writing back via
    /// [`RawProcessingSession::set_parameters_raw`].
    #[must_use]
    pub fn key(&self) -> Option<String> {
        unsafe { self.cf_string(ffi::kVTRAWProcessingParameter_Key) }
    }

    /// Localised, human-readable name.
    #[must_use]
    pub fn name(&self) -> Option<String> {
        unsafe { self.cf_string(ffi::kVTRAWProcessingParameter_Name) }
    }

    /// Long-form description.
    #[must_use]
    pub fn description(&self) -> Option<String> {
        unsafe { self.cf_string(ffi::kVTRAWProcessingParameter_Description) }
    }

    /// `kVTRAWProcessingParameterValueType_*` discriminator.
    #[must_use]
    pub fn value_type(&self) -> Option<String> {
        unsafe { self.cf_string(ffi::kVTRAWProcessingParameter_ValueType) }
    }

    /// Current value (numeric).
    #[must_use]
    pub fn current_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_CurrentValue) }
    }

    /// Minimum value (numeric).
    #[must_use]
    pub fn minimum_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_MinimumValue) }
    }

    /// Maximum value (numeric).
    #[must_use]
    pub fn maximum_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_MaximumValue) }
    }

    /// Initial/default value.
    #[must_use]
    pub fn initial_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_InitialValue) }
    }

    /// Camera-captured value.
    #[must_use]
    pub fn camera_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_CameraValue) }
    }

    /// Neutral (no-op) value.
    #[must_use]
    pub fn neutral_value(&self) -> Option<f64> {
        unsafe { self.cf_f64(ffi::kVTRAWProcessingParameter_NeutralValue) }
    }

    unsafe fn cf_string(&self, key: ffi::CFStringRef) -> Option<String> {
        let v = unsafe { ffi::CFDictionaryGetValue(self.dict, key.cast()) };
        if v.is_null() {
            return None;
        }
        let len = unsafe { ffi::CFStringGetLength(v.cast()) };
        if len < 0 {
            return None;
        }
        let cap = (len as usize).saturating_mul(4) + 1;
        let mut buf = vec![0u8; cap];
        let ok = unsafe {
            ffi::CFStringGetCString(
                v.cast(),
                buf.as_mut_ptr().cast(),
                cap as isize,
                ffi::kCFStringEncodingUTF8,
            )
        };
        if !ok {
            return None;
        }
        let nul = buf.iter().position(|&b| b == 0).unwrap_or(cap);
        Some(String::from_utf8_lossy(&buf[..nul]).into_owned())
    }

    unsafe fn cf_f64(&self, key: ffi::CFStringRef) -> Option<f64> {
        let v = unsafe { ffi::CFDictionaryGetValue(self.dict, key.cast()) };
        if v.is_null() {
            return None;
        }
        let mut out: f64 = 0.0;
        let ok = unsafe {
            ffi::CFNumberGetValue(v.cast(), ffi::kCFNumberFloat64Type, (&raw mut out).cast())
        };
        if ok {
            Some(out)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc, Barrier,
    };
    use std::time::Duration;

    use super::{
        raw_parameter_changed_context_release, raw_parameter_changed_trampoline,
        ParameterChangedContext, ParameterChangedHandlerState,
    };

    struct DropProbe(Arc<AtomicBool>);

    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Release);
        }
    }

    fn empty_context() -> Arc<ParameterChangedContext> {
        Arc::new(ParameterChangedContext::new(Box::new(|_| {})))
    }

    #[test]
    fn handler_transitions_serialize_native_updates() {
        let handler = Arc::new(ParameterChangedHandlerState {
            current: std::sync::Mutex::new(None),
        });
        let start = Arc::new(Barrier::new(3));
        let active = Arc::new(AtomicUsize::new(0));
        let maximum_active = Arc::new(AtomicUsize::new(0));
        let mut threads = Vec::new();

        for _ in 0..2 {
            let handler = Arc::clone(&handler);
            let start = Arc::clone(&start);
            let active = Arc::clone(&active);
            let maximum_active = Arc::clone(&maximum_active);
            threads.push(std::thread::spawn(move || {
                start.wait();
                let status = handler.transition(Some(empty_context()), || {
                    let now = active.fetch_add(1, Ordering::AcqRel) + 1;
                    maximum_active.fetch_max(now, Ordering::AcqRel);
                    std::thread::sleep(Duration::from_millis(20));
                    active.fetch_sub(1, Ordering::AcqRel);
                    0
                });
                assert_eq!(status, 0);
            }));
        }

        start.wait();
        for thread in threads {
            thread.join().expect("transition thread panicked");
        }
        assert_eq!(maximum_active.load(Ordering::Acquire), 1);
    }

    #[test]
    fn failed_native_transition_keeps_current_callback() {
        let current = empty_context();
        let handler = ParameterChangedHandlerState {
            current: std::sync::Mutex::new(Some(Arc::clone(&current))),
        };

        let status = handler.transition(Some(empty_context()), || -7);

        assert_eq!(status, -7);
        let installed = handler
            .current
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(Arc::ptr_eq(
            installed.as_ref().expect("current callback must remain"),
            &current
        ));
        drop(installed);
    }

    #[test]
    fn native_context_owner_keeps_in_flight_callback_alive() {
        let dropped = Arc::new(AtomicBool::new(false));
        let probe = DropProbe(Arc::clone(&dropped));
        let entered = Arc::new(Barrier::new(2));
        let release = Arc::new(Barrier::new(2));
        let callback_entered = Arc::clone(&entered);
        let callback_release = Arc::clone(&release);
        let context = Arc::new(ParameterChangedContext::new(Box::new(move |_| {
            let _ = &probe;
            callback_entered.wait();
            callback_release.wait();
        })));
        let native_context = Arc::into_raw(Arc::clone(&context))
            .cast_mut()
            .cast::<core::ffi::c_void>() as usize;

        drop(context);
        let callback_thread = std::thread::spawn(move || unsafe {
            raw_parameter_changed_trampoline(
                native_context as *mut core::ffi::c_void,
                core::ptr::null(),
            );
        });

        entered.wait();
        assert!(!dropped.load(Ordering::Acquire));
        release.wait();
        callback_thread.join().expect("callback thread panicked");

        unsafe {
            raw_parameter_changed_context_release(native_context as *mut core::ffi::c_void);
        }
        assert!(dropped.load(Ordering::Acquire));
    }

    #[test]
    fn callback_reentrancy_does_not_deadlock() {
        let (completed_tx, completed_rx) = mpsc::channel();
        let calls = Arc::new(AtomicUsize::new(0));
        let context = Arc::new_cyclic(|weak: &std::sync::Weak<ParameterChangedContext>| {
            let weak = weak.clone();
            let calls = Arc::clone(&calls);
            ParameterChangedContext::new(Box::new(move |_| {
                if calls.fetch_add(1, Ordering::AcqRel) == 0 {
                    weak.upgrade()
                        .expect("callback context must still be alive")
                        .invoke(Vec::new());
                    completed_tx
                        .send(())
                        .expect("completion receiver must still be alive");
                }
            }))
        });
        let native_context = Arc::into_raw(Arc::clone(&context))
            .cast_mut()
            .cast::<core::ffi::c_void>() as usize;

        let callback_thread = std::thread::spawn(move || unsafe {
            let native_context = native_context as *mut core::ffi::c_void;
            raw_parameter_changed_trampoline(native_context, core::ptr::null());
            raw_parameter_changed_context_release(native_context);
        });

        completed_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("reentrant callback deadlocked");
        callback_thread.join().expect("callback thread panicked");
        assert_eq!(calls.load(Ordering::Acquire), 1);
    }
}
