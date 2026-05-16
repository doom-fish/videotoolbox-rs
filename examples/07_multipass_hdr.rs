//! Smoke test: create a `FrameSilo` + `MultiPassStorage` +
//! `HdrMetadataSession` and register pro-video workflow decoders.

use videotoolbox::{
    register_professional_workflow_decoders, register_professional_workflow_encoders, FrameSilo,
    HdrMetadataSession, MultiPassStorage,
};

fn main() {
    let silo = FrameSilo::new().expect("create FrameSilo");
    println!("✅ FrameSilo created at {:p}", silo.as_ptr());

    let storage = MultiPassStorage::new().expect("create MultiPassStorage");
    println!("✅ MultiPassStorage created at {:p}", storage.as_ptr());

    let hdr = HdrMetadataSession::new(30.0).expect("create HdrMetadataSession");
    println!("✅ HdrMetadataSession (30 fps) created at {:p}", hdr.as_ptr());

    register_professional_workflow_decoders();
    register_professional_workflow_encoders();
    println!("✅ Pro-video workflow decoders + encoders registered");
}
