#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{driver::derive::RadiationPressure, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithRadiationPressure(
    m: ModulationPtr,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    RadiationPressure::new(*take!(m, Box<M>))
        .with_loop_behavior(loop_behavior.into())
        .into()
}
