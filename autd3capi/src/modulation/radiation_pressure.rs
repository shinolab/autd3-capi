#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{driver::derive::RadiationPressure, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithRadiationPressure(m: ModulationPtr) -> ModulationPtr {
    RadiationPressure::new(*take!(m, Box<M>)).into()
}
