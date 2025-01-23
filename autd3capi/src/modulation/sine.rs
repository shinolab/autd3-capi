#![allow(clippy::missing_safety_doc)]

use autd3::prelude::{rad, SamplingConfig};
use autd3capi_driver::{autd3::modulation::Sine, driver::defined::Hz, *};

#[repr(C)]
pub struct SineOption {
    pub intensity: u8,
    pub offset: u8,
    pub phase_rad: f32,
    pub clamp: bool,
    pub sampling_config: SamplingConfig,
}

impl From<SineOption> for autd3::modulation::SineOption {
    fn from(option: SineOption) -> Self {
        autd3::modulation::SineOption {
            intensity: option.intensity,
            offset: option.offset,
            phase: option.phase_rad * rad,
            clamp: option.clamp,
            sampling_config: option.sampling_config,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExact(freq: u32, option: SineOption) -> ModulationPtr {
    Sine {
        freq: freq * Hz,
        option: option.into(),
    }
    .into()
}

#[no_mangle]
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

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineNearest(freq: f32, option: SineOption) -> ModulationPtr {
    Sine {
        freq: freq * Hz,
        option: option.into(),
    }
    .into_nearest()
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineIsDefault(option: SineOption) -> bool {
    autd3::modulation::SineOption::default() == option.into()
}
