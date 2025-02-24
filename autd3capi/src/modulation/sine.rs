#![allow(clippy::missing_safety_doc)]

use std::num::NonZeroU16;

use autd3capi_driver::{
    autd3::{modulation::Sine, prelude::SamplingConfig},
    driver::defined::{Angle, Hz},
    *,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct SineOption {
    pub intensity: u8,
    pub offset: u8,
    pub phase: Angle,
    pub clamp: bool,
    pub sampling_config_div: u16,
}

impl From<SineOption> for autd3::modulation::SineOption {
    fn from(option: SineOption) -> Self {
        Self {
            intensity: option.intensity,
            offset: option.offset,
            phase: option.phase,
            clamp: option.clamp,
            sampling_config: SamplingConfig::new(unsafe {
                NonZeroU16::new_unchecked(option.sampling_config_div)
            }),
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExact(freq: u32, option: SineOption) -> ModulationPtr {
    Sine {
        freq: freq * Hz,
        option: option.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExactFloat(
    freq: f32,
    option: SineOption,
) -> ModulationPtr {
    Sine {
        freq: freq * Hz,
        option: option.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineNearest(freq: f32, option: SineOption) -> ModulationPtr {
    Sine {
        freq: freq * Hz,
        option: option.into(),
    }
    .into_nearest()
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineIsDefault(option: SineOption) -> bool {
    autd3::modulation::SineOption::default() == option.into()
}
