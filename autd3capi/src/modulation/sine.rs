#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::modulation::{sampling_mode::ExactFreqFloat, Sine},
    driver::{defined::Hz, derive::ModulationProperty},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExact(
    freq: u32,
    config: SamplingConfigWrap,
    intensity: u8,
    offset: u8,
    phase: f64,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Sine::new(freq * Hz)
        .with_sampling_config(config.into())
        .with_intensity(intensity)
        .with_offset(offset)
        .with_phase(phase * autd3::derive::rad)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExactFloat(
    freq: f64,
    config: SamplingConfigWrap,
    intensity: u8,
    offset: u8,
    phase: f64,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Sine::new(freq * Hz)
        .with_sampling_config(config.into())
        .with_intensity(intensity)
        .with_offset(offset)
        .with_phase(phase * autd3::derive::rad)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineNearest(
    freq: f64,
    config: SamplingConfigWrap,
    intensity: u8,
    offset: u8,
    phase: f64,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Sine::with_freq_nearest(freq * Hz)
        .with_sampling_config(config.into())
        .with_intensity(intensity)
        .with_offset(offset)
        .with_phase(phase * autd3::derive::rad)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineIsDefault(sine: ModulationPtr) -> bool {
    let m = take_mod!(sine, Sine<ExactFreqFloat>);
    let default = Sine::new(0. * Hz);
    m.intensity() == default.intensity()
        && m.offset() == default.offset()
        && m.phase() == default.phase()
        && m.sampling_config() == default.sampling_config()
        && m.loop_behavior() == default.loop_behavior()
}
