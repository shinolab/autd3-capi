#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::modulation::{Sine, SineOption},
    driver::defined::Hz,
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineExact(freq: u32, option: SineOption) -> ModulationPtr {
    Sine {
        freq: freq * Hz,
        option,
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
        option,
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineNearest(freq: f32, option: SineOption) -> ModulationPtr {
    Sine {
        freq: freq * Hz,
        option,
    }
    .into_nearest()
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineIsDefault(option: SineOption) -> bool {
    autd3::modulation::SineOption::default() == option
}
