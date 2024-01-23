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
    ModulationPtr::new(
        Sine::new(freq)
            .with_sampling_config(config.into())
            .with_intensity(intensity)
            .with_offset(offset)
            .with_phase(Phase::new(phase))
            .with_mode(mode.into()),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineDefaultSamplingConfig() -> SamplingConfiguration {
    Sine::new(0.0).sampling_config().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineDefaultIntensity() -> u8 {
    Sine::new(0.0).intensity().value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineDefaultPhase() -> u8 {
    Sine::new(0.0).phase().value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineDefaultOffset() -> u8 {
    Sine::new(0.0).offset().value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineDefaultMode() -> SamplingMode {
    Sine::new(0.0).mode().into()
}
