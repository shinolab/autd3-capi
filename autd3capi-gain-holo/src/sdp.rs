#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintPtr};
use autd3_gain_holo::*;
use autd3capi_def::{driver::geometry::Vector3, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDP(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    alpha: float,
    lambda: float,
    repeat: u32,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        create_holo!(SDP, NalgebraBackend, backend, points, amps, size)
            .with_alpha(alpha)
            .with_lambda(lambda)
            .with_repeat(repeat as _)
            .with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPDefaultConstraint() -> EmissionConstraintPtr {
    SDP::new(NalgebraBackend::new().unwrap())
        .constraint()
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPDefaultAlpha() -> float {
    SDP::new(NalgebraBackend::new().unwrap()).alpha()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPDefaultLambda() -> float {
    SDP::new(NalgebraBackend::new().unwrap()).lambda()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPDefaultRepeat() -> u32 {
    SDP::new(NalgebraBackend::new().unwrap()).repeat() as _
}
