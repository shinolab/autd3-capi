#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::modulation::Static, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStatic(intensity: u8) -> ModulationPtr {
    Static::with_intensity(intensity).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStaticIsDefault(s: ModulationPtr) -> bool {
    let m = take_gain!(s, Static);
    let default = Static::new();
    m.intensity() == default.intensity()
}
