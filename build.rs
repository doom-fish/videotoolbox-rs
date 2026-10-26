// videotoolbox build.rs — links the system VideoToolbox framework,
// and (when the `frame_processor` feature is enabled) builds the
// VideoToolboxBridge Swift static library so we can reach the
// VTFrameProcessor Objective-C-only API.

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=swift-bridge");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-env-changed=SDKROOT");
    println!("cargo:rerun-if-env-changed=DEVELOPER_DIR");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    // VideoToolbox is a pure C framework — no Swift bridge required for
    // the core compression/decompression/transfer/rotation APIs.
    println!("cargo:rustc-link-lib=framework=VideoToolbox");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=CoreVideo");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-arg=-Wl,-weak_framework,VideoToolbox");
    println!("cargo:rustc-link-arg=-Wl,-weak_framework,CoreMedia");

    if env::var("CARGO_FEATURE_FRAME_PROCESSOR").is_ok() {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=Metal");
        build_swift_bridge();
    }
}

fn build_swift_bridge() {
    use std::process::Command;

    let swift_dir = "swift-bridge";
    let out_dir = env::var("OUT_DIR").unwrap();
    let swift_build_dir = format!("{out_dir}/swift-build");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let swift_triple = match target_arch.as_str() {
        "x86_64" => "x86_64-apple-macosx",
        "aarch64" => "arm64-apple-macosx",
        other => panic!("videotoolbox: unsupported target arch '{other}'"),
    };

    let swift_args = vec![
        "build",
        "-c",
        "release",
        "--triple",
        swift_triple,
        "--package-path",
        swift_dir,
        "--scratch-path",
        &swift_build_dir,
    ];

    let mut swift_build = Command::new("swift");
    swift_build.args(&swift_args).args(["--jobs", "4"]);
    let output = swift_build.output().expect("Failed to build Swift bridge");
    assert!(
        output.status.success(),
        "Swift build failed ({}):\n{}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let output = swift_build
        .arg("--show-bin-path")
        .output()
        .expect("failed to query the Swift product path");
    assert!(
        output.status.success(),
        "Swift product path query failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let swift_bin_dir = String::from_utf8(output.stdout).expect("Swift product path must be UTF-8");
    let swift_bin_dir = swift_bin_dir.trim();
    assert!(!swift_bin_dir.is_empty(), "Swift product path is empty");

    println!("cargo:rustc-link-search=native={swift_bin_dir}");
    println!("cargo:rustc-link-lib=static=VideoToolboxBridge");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    if let Ok(output) = Command::new("xcode-select").arg("-p").output() {
        if output.status.success() {
            let xcode_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let swift_lib_path =
                format!("{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{swift_lib_path}");
            println!("cargo:rustc-link-search=native={swift_lib_path}");
        }
    }
}
