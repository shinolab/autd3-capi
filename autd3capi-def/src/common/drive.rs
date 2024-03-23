pub const DEFAULT_CORRECTED_ALPHA: f64 = 0.803;

#[cfg(feature = "export")]
mod export {
    use autd3_driver::common::Phase;

    #[no_mangle]
    #[must_use]
    pub unsafe extern "C" fn AUTDEmitIntensityWithCorrectionAlpha(value: u8, alpha: f64) -> u8 {
        autd3_driver::common::EmitIntensity::with_correction_alpha(value, alpha).value()
    }

    #[no_mangle]
    #[must_use]
    pub unsafe extern "C" fn AUTDPhaseFromRad(value: f64) -> u8 {
        Phase::from_rad(value).value()
    }

    #[no_mangle]
    #[must_use]
    pub unsafe extern "C" fn AUTDPhaseToRad(value: u8) -> f64 {
        Phase::new(value).radian()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Drive {
    pub phase: u8,
    pub intensity: u8,
}
