use autd3::{derive::LoopBehavior, prelude::IntoRadiationPressure};
use autd3capi_driver::*;
use driver::datagram::BoxedModulation;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithRadiationPressure(
    m: ModulationPtr,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    take!(m, BoxedModulation)
        .with_radiation_pressure()
        .with_loop_behavior(loop_behavior)
        .into()
}
