//! ABI layout assertions for the `#[repr(C)]` structs shared with the C
//! `VideoToolbox` ABI.
//!
//! `VTDecompressionOutputCallbackRecord` is passed by value into
//! `VTDecompressionSessionCreate`. If its size or alignment ever drifts from
//! what `VideoToolbox` expects, the decoder callback registration silently
//! corrupts. These tests pin the layout so accidental field reordering / type
//! changes are caught at `cargo test` time rather than as runtime garbage.

use std::mem::{align_of, size_of};

use videotoolbox::ffi::{vt_verify_ffi_layout, VTDecompressionOutputCallbackRecord};

#[test]
fn vt_decompression_output_callback_record_layout() {
    // A function pointer followed by a `void *`: two pointer-sized fields.
    let ptr = size_of::<*mut std::ffi::c_void>();
    assert_eq!(
        size_of::<VTDecompressionOutputCallbackRecord>(),
        2 * ptr,
        "VTDecompressionOutputCallbackRecord size drifted"
    );
    assert_eq!(
        align_of::<VTDecompressionOutputCallbackRecord>(),
        align_of::<*mut std::ffi::c_void>(),
        "VTDecompressionOutputCallbackRecord alignment drifted"
    );
}

/// ABI check: confirms the pinned `#[repr(C)]` layouts on the Rust side match
/// what the C `VideoToolbox` ABI expects. A `false` return means the binding
/// layout genuinely disagrees, which is a real ABI bug.
#[test]
fn ffi_layout_matches_abi() {
    assert!(
        vt_verify_ffi_layout(),
        "VideoToolbox FFI struct layout disagrees with the expected C ABI"
    );
}
