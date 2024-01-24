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
) -> ModulationPtr {
    Square::new(freq)
        .with_sampling_config(config.into())
        .with_low(low)
        .with_high(high)
        .with_duty(duty)
        .with_mode(mode.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareDefaultLow() -> u8 {
    Square::new(0.0).low().value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareDefaultHigh() -> u8 {
    Square::new(0.0).high().value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareDefaultDuty() -> float {
    Square::new(0.0).duty()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareDefaultSamplingConfig() -> SamplingConfiguration {
    Square::new(0.0).sampling_config().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareDefaultMode() -> SamplingMode {
    Square::new(0.0).mode().into()
}
