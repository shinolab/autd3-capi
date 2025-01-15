#![allow(clippy::missing_safety_doc)]

use std::num::NonZeroU8;

use crate::{create_holo, EmissionConstraintWrap};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedySphere(
    points: *const Point3,
    amps: *const f32,
    size: u32,
    div: u8,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(Greedy, Sphere, points, amps, size)
        .with_phase_div(NonZeroU8::new_unchecked(div))
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyT4010A1(
    points: *const Point3,
    amps: *const f32,
    size: u32,
    div: u8,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(Greedy, T4010A1, points, amps, size)
        .with_phase_div(NonZeroU8::new_unchecked(div))
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGreedyIsDefault(
    constraint: EmissionConstraintWrap,
    phase_div: u8,
) -> bool {
    let default = Greedy::<Sphere>::new([]);
    let constraint: EmissionConstraint = constraint.into();
    constraint == default.constraint() && phase_div == default.phase_div().get()
}
