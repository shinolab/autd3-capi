#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{autd3::modulation::Static, driver::firmware::fpga::EmitIntensity, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStatic(intensity: EmitIntensity) -> ModulationPtr {
    Static { intensity }.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStaticIsDefault(intensity: EmitIntensity) -> bool {
    let default = Static::default();
    intensity == default.intensity
}
