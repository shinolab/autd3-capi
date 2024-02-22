#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::modulation::Static, *};

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
pub unsafe extern "C" fn AUTDModulationStaticIsDefault(s: ModulationPtr) -> bool {
    let m = take_gain!(s, Static);
    let default = Static::new();
    m.intensity() == default.intensity()
}
