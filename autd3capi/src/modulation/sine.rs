#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::{
        derive::SamplingConfig,
        modulation::{
            sampling_mode::{ExactFreq, ExactFreqFloat, NearestFreq},
            Sine,
        },
    },
    driver::{defined::Hz, derive::ModulationProperty},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExact(
    freq: u32,
    config: SamplingConfigPtr,
    intensity: u8,
    offset: u8,
    phase: f64,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Sine::new(freq * Hz)
        .with_sampling_config(*take!(config, SamplingConfig))
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
    config: SamplingConfigPtr,
    intensity: u8,
    offset: u8,
    phase: f64,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Sine::new(freq * Hz)
        .with_sampling_config(*take!(config, SamplingConfig))
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
    config: SamplingConfigPtr,
    intensity: u8,
    offset: u8,
    phase: f64,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Sine::with_freq_nearest(freq * Hz)
        .with_sampling_config(*take!(config, SamplingConfig))
        .with_intensity(intensity)
        .with_offset(offset)
        .with_phase(phase * autd3::derive::rad)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExactIsDefault(sine: ModulationPtr) -> bool {
    let m = take_gain!(sine, Sine<ExactFreq>);
    let default = Sine::new(0 * Hz);
    m.intensity() == default.intensity()
        && m.offset() == default.offset()
        && m.phase() == default.phase()
        && m.sampling_config() == default.sampling_config()
        && m.loop_behavior() == default.loop_behavior()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExactFloatIsDefault(sine: ModulationPtr) -> bool {
    let m = take_gain!(sine, Sine<ExactFreqFloat>);
    let default = Sine::new(0. * Hz);
    m.intensity() == default.intensity()
        && m.offset() == default.offset()
        && m.phase() == default.phase()
        && m.sampling_config() == default.sampling_config()
        && m.loop_behavior() == default.loop_behavior()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineNearestIsDefault(sine: ModulationPtr) -> bool {
    let m = take_gain!(sine, Sine<NearestFreq>);
    let default = Sine::with_freq_nearest(0. * Hz);
    m.intensity() == default.intensity()
        && m.offset() == default.offset()
        && m.phase() == default.phase()
        && m.sampling_config() == default.sampling_config()
        && m.loop_behavior() == default.loop_behavior()
}
