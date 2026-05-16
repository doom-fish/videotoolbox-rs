#![allow(clippy::cast_possible_wrap)]

//! `VTFrameProcessor` capability queries — `isSupported` + supported
//! scale factors via the new `frame_processor` Swift bridge.
//!
//! Apple's `VTFrameProcessor*Configuration` classes are
//! Objective-C-only — the pure-C FFI used by the rest of this crate
//! can't reach them. This module is feature-gated behind
//! `frame_processor` so the extra `swift build` step only happens
//! when callers opt in.

extern "C" {
    fn vt_super_resolution_is_supported() -> bool;
    fn vt_motion_blur_is_supported() -> bool;
    fn vt_temporal_noise_filter_is_supported() -> bool;
    fn vt_frame_rate_conversion_is_supported() -> bool;
    fn vt_low_latency_super_resolution_is_supported() -> bool;
    fn vt_low_latency_frame_interpolation_is_supported() -> bool;
    fn vt_optical_flow_is_supported() -> bool;
    fn vt_super_resolution_supported_scale_factors(out_buf: *mut u32, max: usize) -> usize;

    fn vt_super_resolution_start(
        frame_width: isize,
        frame_height: isize,
        scale_factor: isize,
        use_precomputed_flow: bool,
        input_is_image: bool,
        out: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn vt_motion_blur_start(
        frame_width: isize,
        frame_height: isize,
        use_precomputed_flow: bool,
        out: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn vt_temporal_noise_filter_start(
        frame_width: isize,
        frame_height: isize,
        source_pixel_format: u32,
        out: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn vt_frame_rate_conversion_start(
        frame_width: isize,
        frame_height: isize,
        use_precomputed_flow: bool,
        out: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn vt_low_latency_super_resolution_start(
        frame_width: isize,
        frame_height: isize,
        scale_factor: f32,
        out: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn vt_low_latency_frame_interpolation_start(
        frame_width: isize,
        frame_height: isize,
        number_of_interpolated_frames: isize,
        out: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn vt_optical_flow_start(
        frame_width: isize,
        frame_height: isize,
        out: *mut *mut core::ffi::c_void,
    ) -> i32;

    fn vt_frame_processor_end(processor: *mut core::ffi::c_void);
    fn vt_frame_processor_release(processor: *mut core::ffi::c_void);
}

/// Per-effect availability snapshot.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct FrameProcessorCapabilities {
    pub super_resolution: bool,
    pub motion_blur: bool,
    pub temporal_noise_filter: bool,
    pub frame_rate_conversion: bool,
    pub low_latency_super_resolution: bool,
    pub low_latency_frame_interpolation: bool,
    pub optical_flow: bool,
}

/// Query Apple's `VTFrameProcessor*Configuration.isSupported` for
/// each effect this binding wraps.
#[must_use]
pub fn frame_processor_capabilities() -> FrameProcessorCapabilities {
    unsafe {
        FrameProcessorCapabilities {
            super_resolution: vt_super_resolution_is_supported(),
            motion_blur: vt_motion_blur_is_supported(),
            temporal_noise_filter: vt_temporal_noise_filter_is_supported(),
            frame_rate_conversion: vt_frame_rate_conversion_is_supported(),
            low_latency_super_resolution: vt_low_latency_super_resolution_is_supported(),
            low_latency_frame_interpolation: vt_low_latency_frame_interpolation_is_supported(),
            optical_flow: vt_optical_flow_is_supported(),
        }
    }
}

/// Return the set of integer scale factors that
/// `VTSuperResolutionScalerConfiguration` accepts on this system
/// (e.g. `vec![2, 4]`). Empty if the API isn't available.
#[must_use]
pub fn super_resolution_supported_scale_factors() -> Vec<u32> {
    const MAX: usize = 16;
    let mut buf = vec![0u32; MAX];
    let n = unsafe { vt_super_resolution_supported_scale_factors(buf.as_mut_ptr(), MAX) };
    buf.truncate(n);
    buf
}

/// An active `VTFrameProcessor` session, started for exactly one
/// filter pipeline. Call one of the `start_*` constructors to begin,
/// then drop the value to end the session.
pub struct FrameProcessor {
    inner: *mut core::ffi::c_void,
}

unsafe impl Send for FrameProcessor {}
unsafe impl Sync for FrameProcessor {}

impl Drop for FrameProcessor {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                vt_frame_processor_end(self.inner);
                vt_frame_processor_release(self.inner);
            }
            self.inner = core::ptr::null_mut();
        }
    }
}

impl FrameProcessor {
    fn from_status(status: i32, ptr: *mut core::ffi::c_void) -> Result<Self, crate::VTError> {
        if status != 0 || ptr.is_null() {
            return Err(crate::VTError::SessionCreateFailed(status));
        }
        Ok(Self { inner: ptr })
    }

    /// Start a session configured for the super-resolution scaler.
    /// Available on macOS 26+.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`](crate::VTError) on failure.
    pub fn start_super_resolution(
        frame_width: usize,
        frame_height: usize,
        scale_factor: usize,
        use_precomputed_flow: bool,
        input_is_image: bool,
    ) -> Result<Self, crate::VTError> {
        let mut out: *mut core::ffi::c_void = core::ptr::null_mut();
        let s = unsafe {
            vt_super_resolution_start(
                frame_width as isize,
                frame_height as isize,
                scale_factor as isize,
                use_precomputed_flow,
                input_is_image,
                &mut out,
            )
        };
        Self::from_status(s, out)
    }

    /// Start a motion-blur session. Available on macOS 15.4+.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`](crate::VTError) on failure.
    pub fn start_motion_blur(
        frame_width: usize,
        frame_height: usize,
        use_precomputed_flow: bool,
    ) -> Result<Self, crate::VTError> {
        let mut out: *mut core::ffi::c_void = core::ptr::null_mut();
        let s = unsafe {
            vt_motion_blur_start(
                frame_width as isize,
                frame_height as isize,
                use_precomputed_flow,
                &mut out,
            )
        };
        Self::from_status(s, out)
    }

    /// Start a temporal-noise-filter session. Available on macOS 26+.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`](crate::VTError) on failure.
    pub fn start_temporal_noise_filter(
        frame_width: usize,
        frame_height: usize,
        source_pixel_format: u32,
    ) -> Result<Self, crate::VTError> {
        let mut out: *mut core::ffi::c_void = core::ptr::null_mut();
        let s = unsafe {
            vt_temporal_noise_filter_start(
                frame_width as isize,
                frame_height as isize,
                source_pixel_format,
                &mut out,
            )
        };
        Self::from_status(s, out)
    }

    /// Start a frame-rate-conversion session. Available on macOS 15.4+.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`](crate::VTError) on failure.
    pub fn start_frame_rate_conversion(
        frame_width: usize,
        frame_height: usize,
        use_precomputed_flow: bool,
    ) -> Result<Self, crate::VTError> {
        let mut out: *mut core::ffi::c_void = core::ptr::null_mut();
        let s = unsafe {
            vt_frame_rate_conversion_start(
                frame_width as isize,
                frame_height as isize,
                use_precomputed_flow,
                &mut out,
            )
        };
        Self::from_status(s, out)
    }

    /// Start a low-latency super-resolution session. Available on
    /// macOS 26+.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`](crate::VTError) on failure.
    pub fn start_low_latency_super_resolution(
        frame_width: usize,
        frame_height: usize,
        scale_factor: f32,
    ) -> Result<Self, crate::VTError> {
        let mut out: *mut core::ffi::c_void = core::ptr::null_mut();
        let s = unsafe {
            vt_low_latency_super_resolution_start(
                frame_width as isize,
                frame_height as isize,
                scale_factor,
                &mut out,
            )
        };
        Self::from_status(s, out)
    }

    /// Start a low-latency frame-interpolation session. Available on
    /// macOS 26+.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`](crate::VTError) on failure.
    pub fn start_low_latency_frame_interpolation(
        frame_width: usize,
        frame_height: usize,
        number_of_interpolated_frames: usize,
    ) -> Result<Self, crate::VTError> {
        let mut out: *mut core::ffi::c_void = core::ptr::null_mut();
        let s = unsafe {
            vt_low_latency_frame_interpolation_start(
                frame_width as isize,
                frame_height as isize,
                number_of_interpolated_frames as isize,
                &mut out,
            )
        };
        Self::from_status(s, out)
    }

    /// Start an optical-flow session. Available on macOS 15.4+.
    ///
    /// # Errors
    ///
    /// Returns [`VTError::SessionCreateFailed`](crate::VTError) on failure.
    pub fn start_optical_flow(
        frame_width: usize,
        frame_height: usize,
    ) -> Result<Self, crate::VTError> {
        let mut out: *mut core::ffi::c_void = core::ptr::null_mut();
        let s =
            unsafe { vt_optical_flow_start(frame_width as isize, frame_height as isize, &mut out) };
        Self::from_status(s, out)
    }

    /// Raw `VTFrameProcessor` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.inner
    }
}
