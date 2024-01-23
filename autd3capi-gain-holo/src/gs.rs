#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintPtr};
use autd3_gain_holo::*;
use autd3capi_def::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGS(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    repeat: u32,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        create_holo!(GS, NalgebraBackend, backend, points, amps, size)
            .with_repeat(repeat as _)
            .with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSDefaultConstraint() -> EmissionConstraintPtr {
    GS::new(NalgebraBackend::new().unwrap()).constraint().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSDefaultRepeat() -> u32 {
    GS::new(NalgebraBackend::new().unwrap()).repeat() as _
}
