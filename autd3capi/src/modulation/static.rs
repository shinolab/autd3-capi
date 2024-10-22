#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{autd3::modulation::Static, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStatic(
    intensity: u8,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Static::with_intensity(intensity)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStaticIsDefault(intensity: u8) -> bool {
    let default = Static::new();
    intensity == default.intensity()
}
