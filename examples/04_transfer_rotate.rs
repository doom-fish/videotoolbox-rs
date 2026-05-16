use apple_cf::cv::CVPixelBuffer;
use videotoolbox::{PixelRotationSession, PixelTransferSession, Rotation};

#[allow(clippy::trivially_copy_pass_by_ref)]
const fn fcc(s: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*s)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bgra = fcc(b"BGRA");
    let nv12 = fcc(b"420v"); // kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange

    // 1) Convert BGRA 1280x720 -> NV12 1280x720
    let src = CVPixelBuffer::create(1280, 720, bgra)
        .map_err(|s| format!("BGRA create failed: {s}"))?;
    let dst = CVPixelBuffer::create(1280, 720, nv12)
        .map_err(|s| format!("NV12 create failed: {s}"))?;

    let xfer = PixelTransferSession::new()?;
    xfer.transfer(&src, &dst)?;
    println!(
        "transferred BGRA->NV12: dst is {}x{} (planar={})",
        dst.width(),
        dst.height(),
        dst.is_planar()
    );

    // 2) Rotate clockwise 90 — dst is 720x1280
    let rsrc = CVPixelBuffer::create(1280, 720, bgra).map_err(|s| format!("src: {s}"))?;
    let rdst = CVPixelBuffer::create(720, 1280, bgra).map_err(|s| format!("dst: {s}"))?;
    let rot = PixelRotationSession::new()?;
    rot.set_rotation(Rotation::Clockwise90)?;
    rot.rotate(&rsrc, &rdst)?;
    println!(
        "rotated CW90: src {}x{} -> dst {}x{}",
        rsrc.width(),
        rsrc.height(),
        rdst.width(),
        rdst.height()
    );
    Ok(())
}
