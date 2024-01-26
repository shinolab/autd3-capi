#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    autd3::modulation::Sine,
    driver::derive::{ModulationProperty, Phase},
    *,
};

use super::SamplingMode;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSine(
    freq: float,
    config: SamplingConfiguration,
    intensity: u8,
    offset: u8,
    phase: u8,
    mode: SamplingMode,
) -> ModulationPtr {
    Sine::new(freq)
        .with_sampling_config(config.into())
        .with_intensity(intensity)
        .with_offset(offset)
        .with_phase(Phase::new(phase))
        .with_mode(mode.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineIsDefault(sine: ModulationPtr) -> bool {
    let m = take_gain!(sine, Sine);
    let default = Sine::new(0.);
    m.intensity() == default.intensity()
        && m.offset() == default.offset()
        && m.phase() == default.phase()
        && m.mode() == default.mode()
        && m.sampling_config() == default.sampling_config()
}
