#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::modulation::{
        sampling_mode::{ExactFreq, ExactFreqFloat, NearestFreq},
        Mixer, Sine,
    },
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationMixerExact(
    components: *const ModulationPtr,
    size: u32,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Mixer::new((0..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine<ExactFreq>)))
        .map(|f| f.with_loop_behavior(loop_behavior.into()))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationMixerExactFloat(
    components: *const ModulationPtr,
    size: u32,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Mixer::new(
        (0..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine<ExactFreqFloat>)),
    )
    .map(|f| f.with_loop_behavior(loop_behavior.into()))
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationMixerNearest(
    components: *const ModulationPtr,
    size: u32,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Mixer::new((0..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine<NearestFreq>)))
        .map(|f| f.with_loop_behavior(loop_behavior.into()))
        .into()
}
