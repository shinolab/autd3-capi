#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::{
        derive::{LoopBehavior, SamplingConfig},
        modulation::Sine,
        prelude::rad,
    },
    driver::{defined::Hz, derive::ModulationProperty},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExact(
    freq: u32,
    config: SamplingConfig,
    intensity: u8,
    offset: u8,
    phase: f32,
    clamp: bool,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Sine::new(freq * Hz)
        .with_intensity(intensity)
        .with_offset(offset)
        .with_phase(phase * rad)
        .with_clamp(clamp)
        .with_loop_behavior(loop_behavior)
        .with_sampling_config(config)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExactFloat(
    freq: f32,
    config: SamplingConfig,
    intensity: u8,
    offset: u8,
    phase: f32,
    clamp: bool,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Sine::new(freq * Hz)
        .with_intensity(intensity)
        .with_offset(offset)
        .with_phase(phase * rad)
        .with_loop_behavior(loop_behavior)
        .with_clamp(clamp)
        .with_sampling_config(config)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineNearest(
    freq: f32,
    config: SamplingConfig,
    intensity: u8,
    offset: u8,
    phase: f32,
    clamp: bool,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Sine::new_nearest(freq * Hz)
        .with_intensity(intensity)
        .with_offset(offset)
        .with_phase(phase * rad)
        .with_clamp(clamp)
        .with_loop_behavior(loop_behavior)
        .with_sampling_config(config)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExactFreq(freq: u32) -> u32 {
    Sine::new(freq * Hz).freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExactFloatFreq(freq: f32) -> f32 {
    Sine::new(freq * Hz).freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineNearestFreq(freq: f32) -> f32 {
    Sine::new_nearest(freq * Hz).freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineIsDefault(
    config: SamplingConfig,
    intensity: u8,
    offset: u8,
    phase: f32,
    clamp: bool,
    loop_behavior: LoopBehavior,
) -> bool {
    let default = Sine::new(0. * Hz);
    intensity == default.intensity()
        && offset == default.offset()
        && phase == default.phase().radian()
        && config == default.sampling_config()
        && loop_behavior == default.loop_behavior()
        && clamp == default.clamp()
}
