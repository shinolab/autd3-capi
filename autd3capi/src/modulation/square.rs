#![allow(clippy::missing_safety_doc)]

use std::num::NonZeroU16;

use autd3capi_driver::{
    autd3::{modulation::Square, prelude::SamplingConfig},
    driver::common::Hz,
    *,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct SquareOption {
    pub low: u8,
    pub high: u8,
    pub duty: f32,
    pub sampling_config_div: u16,
}

impl From<SquareOption> for autd3::modulation::SquareOption {
    fn from(option: SquareOption) -> Self {
        Self {
            low: option.low,
            high: option.high,
            duty: option.duty,
            sampling_config: SamplingConfig::new(unsafe {
                NonZeroU16::new_unchecked(option.sampling_config_div)
            }),
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
