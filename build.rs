use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DOCS_RS");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    // VideoToolbox is a pure C framework — no Swift bridge required. We just
    // link the system frameworks and declare extern "C" bindings in src/ffi.
    println!("cargo:rustc-link-lib=framework=VideoToolbox");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=CoreVideo");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}
