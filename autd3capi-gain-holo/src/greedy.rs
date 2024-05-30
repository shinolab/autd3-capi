#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, EmissionConstraintWrap};
use autd3_gain_holo::*;
use autd3capi_driver::{
    driver::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedySphere(
    points: *const f64,
    amps: *const f64,
    size: u64,
    div: u8,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(Greedy, Sphere, points, amps, size)
        .with_phase_div(div)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyT4010A1(
    points: *const f64,
    amps: *const f64,
    size: u64,
    div: u8,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(Greedy, T4010A1, points, amps, size)
        .with_phase_div(div)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGreedyIsDefault(greedy: GainPtr) -> bool {
    let g = take_gain!(greedy, Greedy<Sphere>);
    let default = Greedy::<Sphere>::new([]);
    g.constraint() == default.constraint() && g.phase_div() == default.phase_div()
}
