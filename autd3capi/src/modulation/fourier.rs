#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::modulation::{
        sampling_mode::{ExactFreq, ExactFreqFloat, NearestFreq},
        Fourier, Sine,
    },
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierExact(
    components: *const ModulationPtr,
    size: u32,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Fourier::new(**take_mod!(components.read(), Sine<ExactFreq>))
        .add_components_from_iter(
            (1..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine<ExactFreq>)),
        )
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierExactFloat(
    components: *const ModulationPtr,
    size: u32,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Fourier::new(**take_mod!(components.read(), Sine<ExactFreqFloat>))
        .add_components_from_iter(
            (1..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine<ExactFreqFloat>)),
        )
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierNearest(
    components: *const ModulationPtr,
    size: u32,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    Fourier::new(**take_mod!(components.read(), Sine<NearestFreq>))
        .add_components_from_iter(
            (1..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine<NearestFreq>)),
        )
        .with_loop_behavior(loop_behavior.into())
        .into()
}
