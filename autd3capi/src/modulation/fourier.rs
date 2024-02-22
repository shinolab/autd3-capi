#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    autd3::modulation::{Fourier, Sine},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourier(
    components: *const ModulationPtr,
    size: u32,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Fourier::new(**take_mod!(components.read(), Sine))
        .add_components_from_iter(
            (1..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine)),
        )
        .with_loop_behavior(loop_behavior.into())
        .into()
}
