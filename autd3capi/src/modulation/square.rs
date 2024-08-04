#![allow(clippy::missing_safety_doc)]

use autd3::{
    derive::SamplingConfig,
    modulation::sampling_mode::{ExactFreq, NearestFreq},
};
use autd3capi_driver::{
    autd3::modulation::{sampling_mode::ExactFreqFloat, Square},
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
        .with_loop_behavior(loop_behavior.into())
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
        .with_loop_behavior(loop_behavior.into())
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
        .with_loop_behavior(loop_behavior.into())
        .with_sampling_config(config)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExactFreq(square: ModulationPtr) -> u32 {
    take_mod!(square, Square<ExactFreq>).freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExactFloatFreq(square: ModulationPtr) -> f32 {
    take_mod!(square, Square<ExactFreqFloat>).freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareNearestFreq(square: ModulationPtr) -> f32 {
    take_mod!(square, Square<NearestFreq>).freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareIsDefault(square: ModulationPtr) -> bool {
    let m = take_mod!(square, Square<ExactFreqFloat>);
    let default = Square::new(0. * Hz);
    m.low() == default.low()
        && m.high() == default.high()
        && m.duty() == default.duty()
        && m.sampling_config() == default.sampling_config()
        && m.loop_behavior() == default.loop_behavior()
}
