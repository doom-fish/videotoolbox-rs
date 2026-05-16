//! API-surface coverage harness for `videotoolbox`.
//!
//! Verifies every `extern "C"` symbol we declare against Apple's
//! `VideoToolbox.framework` headers, and reports the diff:
//!
//! * **Wrapped** — Apple symbol present in our `extern "C"` block.
//! * **Missing** — Apple symbol absent from our wrapper. Either listed in
//!   `intentionally_omitted()` (with a reason) or a real coverage gap.
//! * **Unknown** — Symbol in our `extern "C"` block that doesn't exist in
//!   the SDK headers. Always fails the test.
//!
//! See `tests/api_coverage.rs` in `apple-cf-rs` for the same harness shape.
//! This file now audits the broader `videotoolbox` surface that the crate
//! exposes directly from `src/ffi/mod.rs`.

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun must be available");
    assert!(out.status.success(), "xcrun --show-sdk-path failed");
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn read_headers(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|p| {
            std::fs::read_to_string(p).unwrap_or_else(|e| panic!("can't read {}: {e}", p.display()))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_by_pattern(pattern: &str, source: &str) -> BTreeSet<String> {
    let re = regex_lite::Regex::new(pattern).unwrap();
    re.captures_iter(source).map(|c| c[1].to_string()).collect()
}

fn read_our_ffi() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ffi/mod.rs");
    std::fs::read_to_string(path).unwrap()
}

#[derive(Default)]
struct Report {
    framework: &'static str,
    apple: BTreeSet<String>,
    ours: BTreeSet<String>,
    omitted: BTreeSet<String>,
    bridge_only: BTreeSet<String>,
}

impl Report {
    fn run(self) -> Result<(), String> {
        let wrapped: BTreeSet<&String> = self.apple.intersection(&self.ours).collect();
        let missing: BTreeSet<&String> = self
            .apple
            .difference(&self.ours)
            .filter(|s| !self.omitted.contains(*s))
            .collect();
        let unknown: BTreeSet<&String> = self
            .ours
            .difference(&self.apple)
            .filter(|s| !self.bridge_only.contains(*s))
            .collect();

        let coverable = wrapped.len() + missing.len();
        let pct = if coverable == 0 {
            100.0
        } else {
            wrapped.len() as f64 / coverable as f64 * 100.0
        };

        println!(
            "\n=== {} API coverage ===\n\
             Apple symbols:           {}\n\
             Intentionally omitted:    {}\n\
             Bridge-only (in our FFI): {}\n\
             ----\n\
             Coverable (Apple - omitted): {coverable}\n\
             Wrapped:                  {} ({pct:.1}% of coverable)\n\
             Missing (gap):            {}\n\
             Unknown (stale?):         {}",
            self.framework,
            self.apple.len(),
            self.omitted.len(),
            self.bridge_only.len(),
            wrapped.len(),
            missing.len(),
            unknown.len(),
        );

        if !missing.is_empty() {
            println!("\n--- Missing ---");
            for s in &missing {
                println!("  - {s}");
            }
        }
        if !unknown.is_empty() {
            println!("\n--- Unknown (typo/stale?) ---");
            for s in &unknown {
                println!("  - {s}");
            }
        }

        if !unknown.is_empty() {
            return Err(format!(
                "{} has {} unknown symbol(s) in extern \"C\" — see stdout",
                self.framework,
                unknown.len()
            ));
        }
        if pct < 100.0 {
            return Err(format!(
                "{} coverable coverage is {pct:.1}% — every Apple symbol must be \
                 either wrapped or in intentionally_omitted()",
                self.framework
            ));
        }
        Ok(())
    }
}

/// All `pub fn <name>(` symbol names declared anywhere in `src/ffi/mod.rs`.
fn extract_our_extern_fns() -> BTreeSet<String> {
    extract_by_pattern(r"pub\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(", &read_our_ffi())
}

// ---- Allowlists ----

fn vt_compression_intentionally_omitted() -> BTreeSet<String> {
    [
        // Public entry points we still intentionally skip in the safe layer.
        "VTCompressionSessionEncodeMultiImageFrame",
        "VTCompressionSessionEncodeMultiImageFrameWithOutputHandler",
        "VTCompressionSessionEncodeFrameWithOutputHandler",
        "VTIsStereoMVHEVCEncodeSupported",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

/// Compression property keys that are still intentionally omitted from the
/// safe API surface. The raw constants may exist in `src/ffi/mod.rs`, but this
/// harness only requires typed coverage for the keys the crate actively wraps.
fn vt_property_intentionally_omitted(apple: &BTreeSet<String>) -> BTreeSet<String> {
    let kept: BTreeSet<&str> = [
        "kVTCompressionPropertyKey_RealTime",
        "kVTCompressionPropertyKey_AllowFrameReordering",
        "kVTCompressionPropertyKey_AverageBitRate",
        "kVTCompressionPropertyKey_ExpectedFrameRate",
        "kVTCompressionPropertyKey_MaxKeyFrameInterval",
        "kVTCompressionPropertyKey_ProfileLevel",
        "kVTCompressionPropertyKey_H264EntropyMode",
    ]
    .into_iter()
    .collect();
    apple
        .iter()
        .filter(|k| !kept.contains(k.as_str()))
        .cloned()
        .collect()
}

/// Profile-level constants we still intentionally leave out of the typed
/// `ProfileLevel` enum. Compute by subtracting the kept set from Apple's set.
fn vt_profile_intentionally_omitted(apple: &BTreeSet<String>) -> BTreeSet<String> {
    let kept: BTreeSet<&str> = [
        "kVTProfileLevel_H264_Baseline_AutoLevel",
        "kVTProfileLevel_H264_Main_AutoLevel",
        "kVTProfileLevel_H264_High_AutoLevel",
        "kVTProfileLevel_HEVC_Main_AutoLevel",
        "kVTProfileLevel_HEVC_Main10_AutoLevel",
    ]
    .into_iter()
    .collect();
    apple
        .iter()
        .filter(|k| !kept.contains(k.as_str()))
        .cloned()
        .collect()
}

fn bridge_only_symbols() -> BTreeSet<String> {
    [
        // Type aliases / callback types — not Apple symbols themselves but
        // declared in our ffi for ergonomic Rust signatures.
        "VTCompressionOutputCallback",
        "VTCompressionSession",
        "VTCompressionSessionRef",
        "VTEncodeInfoFlags",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

// ---- Test cases ----

#[test]
fn vt_compression_function_coverage() {
    let sdk = sdk_root();
    let header =
        sdk.join("System/Library/Frameworks/VideoToolbox.framework/Headers/VTCompressionSession.h");
    let apple = extract_by_pattern(
        r"\b(VTCompressionSession[A-Za-z0-9_]+|VTIs[A-Za-z0-9_]+)\s*\(",
        &read_headers(&[header]),
    );
    let ours: BTreeSet<String> = extract_our_extern_fns()
        .into_iter()
        .filter(|n| n.starts_with("VTCompressionSession") || n == "VTIsStereoMVHEVCEncodeSupported")
        .collect();

    Report {
        framework: "VTCompressionSession (functions)",
        apple,
        ours,
        omitted: vt_compression_intentionally_omitted(),
        bridge_only: bridge_only_symbols(),
    }
    .run()
    .unwrap();
}

#[test]
fn vt_session_set_property_function_coverage() {
    let sdk = sdk_root();
    let header = sdk.join("System/Library/Frameworks/VideoToolbox.framework/Headers/VTSession.h");
    let apple = extract_by_pattern(r"\b(VTSession[A-Za-z0-9_]+)\s*\(", &read_headers(&[header]));
    let ours: BTreeSet<String> = extract_our_extern_fns()
        .into_iter()
        .filter(|n| n.starts_with("VTSession"))
        .collect();

    let omitted = BTreeSet::new();

    Report {
        framework: "VTSession (functions)",
        apple,
        ours,
        omitted,
        bridge_only: BTreeSet::new(),
    }
    .run()
    .unwrap();
}

#[test]
fn vt_compression_property_key_coverage() {
    let sdk = sdk_root();
    let header = sdk
        .join("System/Library/Frameworks/VideoToolbox.framework/Headers/VTCompressionProperties.h");
    let apple = extract_by_pattern(
        r"\b(kVTCompressionPropertyKey_[A-Za-z0-9_]+)",
        &read_headers(&[header]),
    );
    let ours = extract_by_pattern(
        r"\b(kVTCompressionPropertyKey_[A-Za-z0-9_]+)",
        &read_our_ffi(),
    );

    Report {
        framework: "VTCompressionProperties (keys)",
        apple: apple.clone(),
        ours,
        omitted: vt_property_intentionally_omitted(&apple),
        bridge_only: BTreeSet::new(),
    }
    .run()
    .unwrap();
}

#[test]
fn vt_profile_level_constant_coverage() {
    let sdk = sdk_root();
    let header = sdk
        .join("System/Library/Frameworks/VideoToolbox.framework/Headers/VTCompressionProperties.h");
    let apple = extract_by_pattern(
        r"\b(kVTProfileLevel_[A-Za-z0-9_]+)",
        &read_headers(&[header]),
    );
    let ours = extract_by_pattern(r"\b(kVTProfileLevel_[A-Za-z0-9_]+)", &read_our_ffi());

    Report {
        framework: "kVTProfileLevel_* constants",
        apple: apple.clone(),
        ours,
        omitted: vt_profile_intentionally_omitted(&apple),
        bridge_only: BTreeSet::new(),
    }
    .run()
    .unwrap();
}

#[test]
fn cm_video_codec_type_coverage() {
    let sdk = sdk_root();
    let header =
        sdk.join("System/Library/Frameworks/CoreMedia.framework/Headers/CMFormatDescription.h");
    let apple = extract_by_pattern(
        r"\b(kCMVideoCodecType_[A-Za-z0-9_]+)",
        &read_headers(&[header]),
    );
    // Constants the Codec enum currently lowers into.
    let ours = extract_by_pattern(r"\b(kCMVideoCodecType_[A-Za-z0-9_]+)", &read_our_ffi());

    // For v0.1 we only wrap the codecs that have a Rust enum variant.
    let kept: BTreeSet<&str> = [
        "kCMVideoCodecType_H264",
        "kCMVideoCodecType_HEVC",
        "kCMVideoCodecType_AppleProRes422",
        "kCMVideoCodecType_AppleProRes422HQ",
        "kCMVideoCodecType_AppleProRes422LT",
        "kCMVideoCodecType_AppleProRes422Proxy",
        "kCMVideoCodecType_AppleProRes4444",
    ]
    .into_iter()
    .collect();
    let omitted: BTreeSet<String> = apple
        .iter()
        .filter(|k| !kept.contains(k.as_str()))
        .cloned()
        .collect();

    Report {
        framework: "kCMVideoCodecType_* constants",
        apple,
        ours,
        omitted,
        bridge_only: BTreeSet::new(),
    }
    .run()
    .unwrap();
}
