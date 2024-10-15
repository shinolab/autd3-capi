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
    clamp: bool,
    scale_factor: f32,
    offset: u8,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Fourier::new((0..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine<ExactFreq>)))
        .map(|f| {
            f.with_clamp(clamp)
                .with_scale_factor(if scale_factor.is_nan() {
                    None
                } else {
                    Some(scale_factor)
                })
                .with_offset(offset)
                .with_loop_behavior(loop_behavior.into())
        })
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierExactFloat(
    components: *const ModulationPtr,
    size: u32,
    clamp: bool,
    scale_factor: f32,
    offset: u8,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Fourier::new(
        (0..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine<ExactFreqFloat>)),
    )
    .map(|f| {
        f.with_clamp(clamp)
            .with_scale_factor(if scale_factor.is_nan() {
                None
            } else {
                Some(scale_factor)
            })
            .with_offset(offset)
            .with_loop_behavior(loop_behavior.into())
    })
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierNearest(
    components: *const ModulationPtr,
    size: u32,
    clamp: bool,
    scale_factor: f32,
    offset: u8,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Fourier::new(
        (0..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine<NearestFreq>)),
    )
    .map(|f| {
        f.with_clamp(clamp)
            .with_scale_factor(if scale_factor.is_nan() {
                None
            } else {
                Some(scale_factor)
            })
            .with_offset(offset)
            .with_loop_behavior(loop_behavior.into())
    })
    .into()
}
