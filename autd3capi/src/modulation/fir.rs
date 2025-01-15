use autd3::{core::modulation::LoopBehavior, modulation::IntoFir};
use autd3capi_driver::*;
use driver::datagram::BoxedModulation;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithFir(
    m: ModulationPtr,
    loop_behavior: LoopBehavior,
    coef: *const f32,
    n_tap: u32,
) -> ModulationPtr {
    take!(m, BoxedModulation)
        .with_fir((0..n_tap as usize).map(|i| coef.add(i).read()))
        .with_loop_behavior(loop_behavior)
        .into()
}
