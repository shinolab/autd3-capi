#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{autd3::modulation::Square, driver::defined::Hz, *};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct SquareOption {
    pub low: u8,
    pub high: u8,
    pub duty: f32,
    pub sampling_config: SamplingConfigWrap,
}

impl From<SquareOption> for autd3::modulation::SquareOption {
    fn from(option: SquareOption) -> Self {
        Self {
            low: option.low,
            high: option.high,
            duty: option.duty,
            sampling_config: option.sampling_config.into(),
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExact(
    freq: u32,
    option: SquareOption,
) -> ModulationPtr {
    Square {
        freq: freq * Hz,
        option: option.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExactFloat(
    freq: f32,
    option: SquareOption,
) -> ModulationPtr {
    Square {
        freq: freq * Hz,
        option: option.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareNearest(
    freq: f32,
    option: SquareOption,
) -> ModulationPtr {
    Square {
        freq: freq * Hz,
        option: option.into(),
    }
    .into_nearest()
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareIsDefault(option: SquareOption) -> bool {
    autd3::modulation::SquareOption::default() == option.into()
}
