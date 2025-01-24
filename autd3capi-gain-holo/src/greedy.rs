#![allow(clippy::missing_safety_doc)]

use std::num::NonZeroU8;

use crate::{create_holo, EmissionConstraintWrap};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Directivity, Sphere, T4010A1},
    *,
};

#[repr(C)]
pub struct GreedyOption {
    pub constraint: EmissionConstraintWrap,
    pub phase_div: u8,
}

impl<T: Directivity> From<GreedyOption> for autd3_gain_holo::GreedyOption<T> {
    fn from(option: GreedyOption) -> Self {
        autd3_gain_holo::GreedyOption {
            constraint: option.constraint.into(),
            phase_div: NonZeroU8::new(option.phase_div).unwrap(),
            __phantom: std::marker::PhantomData,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedySphere(
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GreedyOption,
) -> GainPtr {
    let foci = create_holo!(Sphere, points, amps, size);
    Greedy::<Sphere> {
        foci,
        option: option.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyT4010A1(
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GreedyOption,
) -> GainPtr {
    let foci = create_holo!(T4010A1, points, amps, size);
    Greedy::<T4010A1> {
        foci,
        option: option.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGreedyIsDefault(option: GreedyOption) -> bool {
    autd3_gain_holo::GreedyOption::<Sphere>::default() == option.into()
}
