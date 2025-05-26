#![allow(clippy::missing_safety_doc)]

use std::num::NonZeroU8;

use crate::{EmissionConstraintWrap, create_holo};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Directivity, Sphere, T4010A1},
    *,
};

#[repr(C)]
pub struct GreedyOption {
    pub constraint: EmissionConstraintWrap,
    pub phase_quantization_levels: u8,
}

impl<T: Directivity> From<GreedyOption> for autd3_gain_holo::GreedyOption<T, AbsGreedyObjectiveFn> {
    fn from(option: GreedyOption) -> Self {
        autd3_gain_holo::GreedyOption {
            constraint: option.constraint.into(),
            phase_quantization_levels: NonZeroU8::new(option.phase_quantization_levels).unwrap(),
            objective_func: AbsGreedyObjectiveFn,
            __phantom: std::marker::PhantomData,
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedySphere(
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GreedyOption,
) -> GainPtr {
    let foci = create_holo!(Sphere, points, amps, size);
    Greedy::<Sphere, AbsGreedyObjectiveFn> {
        foci,
        option: option.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyT4010A1(
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GreedyOption,
) -> GainPtr {
    let foci = create_holo!(T4010A1, points, amps, size);
    Greedy::<T4010A1, AbsGreedyObjectiveFn> {
        foci,
        option: option.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainGreedyIsDefault(option: GreedyOption) -> bool {
    autd3_gain_holo::GreedyOption::<Sphere, AbsGreedyObjectiveFn>::default() == option.into()
}
