use videotoolbox::{
    available_video_encoder_details, supported_property_dictionary_for_encoder, Codec,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let encoders = available_video_encoder_details().map_err(|s| format!("status {s}"))?;
    println!("found {} video encoders:", encoders.len());
    for (i, enc) in encoders.iter().enumerate() {
        let fourcc = enc.base.codec_type.to_be_bytes();
        let selection_property_count = enc
            .supported_selection_properties
            .as_ref()
            .map_or(0, apple_cf::cf::CFDictionary::len);
        println!(
            "  {}. {} ({}) [{}, fourcc {:?}, hw={:?}, perf={:?}, quality={:?}, gpu={:?}, selection-keys={}]",
            i + 1,
            enc.base.display_name,
            enc.base.encoder_id,
            enc.base.codec_name,
            String::from_utf8_lossy(&fourcc),
            enc.is_hardware_accelerated,
            enc.performance_rating,
            enc.quality_rating,
            enc.gpu_registry_id,
            selection_property_count,
        );
    }

    let properties = supported_property_dictionary_for_encoder(1920, 1080, Codec::H264, None)
        .map_err(|s| format!("status {s}"))?;
    println!(
        "selected encoder {:?} exposes {} supported properties",
        properties.encoder_id,
        properties
            .supported_properties
            .as_ref()
            .map_or(0, apple_cf::cf::CFDictionary::len)
    );
    Ok(())
}
