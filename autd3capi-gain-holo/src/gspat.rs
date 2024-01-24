#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintPtr};
use autd3_gain_holo::*;
use autd3capi_def::{driver::geometry::Vector3, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPAT(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    repeat: u32,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    create_holo!(GSPAT, NalgebraBackend, backend, points, amps, size)
        .with_repeat(repeat as _)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATDefaultConstraint() -> EmissionConstraintPtr {
    GSPAT::new(NalgebraBackend::new().unwrap())
        .constraint()
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATDefaultRepeat() -> u32 {
    GSPAT::new(NalgebraBackend::new().unwrap()).repeat() as _
}
