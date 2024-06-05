#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::modulation::{sampling_mode::ExactFreqFloat, Square},
    driver::{defined::Hz, derive::ModulationProperty},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExact(
    freq: u32,
    config: SamplingConfigWrap,
    low: u8,
    high: u8,
    duty: f32,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Square::new(freq * Hz)
        .with_sampling_config(config.into())
        .with_low(low)
        .with_high(high)
        .with_duty(duty)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExactFloat(
    freq: f32,
    config: SamplingConfigWrap,
    low: u8,
    high: u8,
    duty: f32,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Square::new(freq * Hz)
        .with_sampling_config(config.into())
        .with_low(low)
        .with_high(high)
        .with_duty(duty)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareNearest(
    freq: f32,
    config: SamplingConfigWrap,
    low: u8,
    high: u8,
    duty: f32,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Square::with_freq_nearest(freq * Hz)
        .with_sampling_config(config.into())
        .with_low(low)
        .with_high(high)
        .with_duty(duty)
        .with_loop_behavior(loop_behavior.into())
        .into()
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
