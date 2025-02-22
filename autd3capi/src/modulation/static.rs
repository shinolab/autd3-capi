#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{autd3::modulation::Static, *};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStatic(intensity: u8) -> ModulationPtr {
    Static { intensity }.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStaticIsDefault(intensity: u8) -> bool {
    let default = Static::default();
    intensity == default.intensity
}
