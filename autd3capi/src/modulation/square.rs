#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::modulation::{Square, SquareOption},
    driver::defined::Hz,
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExact(
    freq: u32,
    option: SquareOption,
) -> ModulationPtr {
    Square {
        freq: freq * Hz,
        option,
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareExactFloat(
    freq: f32,
    option: SquareOption,
) -> ModulationPtr {
    Square {
        freq: freq * Hz,
        option,
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareNearest(
    freq: f32,
    option: SquareOption,
) -> ModulationPtr {
    Square {
        freq: freq * Hz,
        option,
    }
    .into_nearest()
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareIsDefault(option: SquareOption) -> bool {
    option == Default::default()
}
