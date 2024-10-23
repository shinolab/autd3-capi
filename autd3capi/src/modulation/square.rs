#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::{
        derive::{LoopBehavior, SamplingConfig},
        modulation::Square,
    },
    driver::{defined::Hz, derive::ModulationProperty},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExact(
    freq: u32,
    config: SamplingConfig,
    low: u8,
    high: u8,
    duty: f32,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Square::new(freq * Hz)
        .with_low(low)
        .with_high(high)
        .with_duty(duty)
        .with_loop_behavior(loop_behavior)
        .with_sampling_config(config)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExactFloat(
    freq: f32,
    config: SamplingConfig,
    low: u8,
    high: u8,
    duty: f32,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Square::new(freq * Hz)
        .with_low(low)
        .with_high(high)
        .with_duty(duty)
        .with_loop_behavior(loop_behavior)
        .with_sampling_config(config)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareNearest(
    freq: f32,
    config: SamplingConfig,
    low: u8,
    high: u8,
    duty: f32,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Square::new_nearest(freq * Hz)
        .with_low(low)
        .with_high(high)
        .with_duty(duty)
        .with_loop_behavior(loop_behavior)
        .with_sampling_config(config)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExactFreq(freq: u32) -> u32 {
    Square::new(freq * Hz).freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExactFloatFreq(freq: f32) -> f32 {
    Square::new(freq * Hz).freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareNearestFreq(freq: f32) -> f32 {
    Square::new_nearest(freq * Hz).freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareIsDefault(
    config: SamplingConfig,
    low: u8,
    high: u8,
    duty: f32,
    loop_behavior: LoopBehavior,
) -> bool {
    let default = Square::new(0. * Hz);
    low == default.low()
        && high == default.high()
        && duty == default.duty()
        && config == default.sampling_config()
        && loop_behavior == default.loop_behavior()
}
