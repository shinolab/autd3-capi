#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintWrap};
use autd3_gain_holo::*;
use autd3capi_driver::{
    driver::{
        acoustics::directivity::{Sphere, T4010A1},
        geometry::Vector3,
    },
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPSphere(
    backend: BackendPtr,
    points: *const f64,
    amps: *const f64,
    size: u64,
    alpha: f64,
    lambda: f64,
    repeat: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(SDP, NalgebraBackend, Sphere, backend, points, amps, size)
        .with_alpha(alpha)
        .with_lambda(lambda)
        .with_repeat(repeat as _)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPT4010A1(
    backend: BackendPtr,
    points: *const f64,
    amps: *const f64,
    size: u64,
    alpha: f64,
    lambda: f64,
    repeat: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(SDP, NalgebraBackend, T4010A1, backend, points, amps, size)
        .with_alpha(alpha)
        .with_lambda(lambda)
        .with_repeat(repeat as _)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSDPIsDefault(gs: GainPtr) -> bool {
    let g = take_gain!(gs, SDP<Sphere,NalgebraBackend<Sphere>>);
    let default = SDP::new(std::sync::Arc::new(NalgebraBackend::default()));
    g.constraint() == default.constraint()
        && g.alpha() == default.alpha()
        && g.lambda() == default.lambda()
        && g.repeat() == default.repeat()
}
