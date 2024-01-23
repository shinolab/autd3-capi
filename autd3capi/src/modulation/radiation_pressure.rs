#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::modulation::IntoRadiationPressure, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithRadiationPressure(m: ModulationPtr) -> ModulationPtr {
    ModulationPtr::new(Box::from_raw(m.0 as *mut Box<M>).with_radiation_pressure())
}
