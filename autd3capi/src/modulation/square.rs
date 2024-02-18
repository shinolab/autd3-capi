#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::modulation::Square, driver::derive::ModulationProperty, *};

use super::SamplingMode;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquare(
    freq: float,
    config: SamplingConfiguration,
    low: u8,
    high: u8,
    duty: float,
    mode: SamplingMode,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Square::new(freq)
        .with_sampling_config(config.into())
        .with_low(low)
        .with_high(high)
        .with_duty(duty)
        .with_mode(mode.into())
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareIsDefault(square: ModulationPtr) -> bool {
    let m = take_gain!(square, Square);
    let default = Square::new(0.);
    m.low() == default.low()
        && m.high() == default.high()
        && m.duty() == default.duty()
        && m.mode() == default.mode()
        && m.sampling_config() == default.sampling_config()
        && m.loop_behavior() == default.loop_behavior()
}
