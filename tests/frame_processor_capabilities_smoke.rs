#![cfg(feature = "frame_processor")]

use videotoolbox::{
    frame_processor_capabilities, low_latency_super_resolution_supported_scale_factors,
    super_resolution_model_percentage_available, super_resolution_model_status,
    super_resolution_supported_scale_factors,
};

#[test]
fn frame_processor_capabilities_are_self_consistent() {
    let caps = frame_processor_capabilities();
    assert_eq!(
        caps,
        frame_processor_capabilities(),
        "capability query should be stable"
    );

    let scales = super_resolution_supported_scale_factors();
    assert!(
        scales.iter().all(|scale| *scale > 0),
        "scale factors must be positive"
    );

    if let Some(scale) = scales.first().copied() {
        let scale = usize::try_from(scale).expect("scale factor should fit in usize");
        let _status = super_resolution_model_status(640, 360, scale, false, true);
        if let Some(percentage) =
            super_resolution_model_percentage_available(640, 360, scale, false, true)
        {
            assert!(
                (0.0..=100.0).contains(&percentage),
                "model-download percentage should be between 0 and 100, got {percentage}"
            );
        }
    }

    let low_latency_scales = low_latency_super_resolution_supported_scale_factors(640, 360);
    assert!(
        low_latency_scales
            .iter()
            .all(|scale| scale.is_finite() && *scale > 0.0),
        "low-latency scale factors must be finite, positive numbers"
    );

    if !low_latency_scales.is_empty() {
        assert!(caps.low_latency_super_resolution);
    }
}
