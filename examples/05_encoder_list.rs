use videotoolbox::available_video_encoders;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let encoders = available_video_encoders().map_err(|s| format!("status {s}"))?;
    println!("found {} video encoders:", encoders.len());
    for (i, enc) in encoders.iter().enumerate() {
        let fourcc = enc.codec_type.to_be_bytes();
        println!(
            "  {}. {} ({}) [{}, fourcc {:?}]",
            i + 1,
            enc.display_name,
            enc.encoder_id,
            enc.codec_name,
            String::from_utf8_lossy(&fourcc)
        );
    }
    Ok(())
}
