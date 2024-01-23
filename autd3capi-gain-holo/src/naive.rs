#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintPtr};
use autd3_gain_holo::*;
use autd3capi_def::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaive(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        create_holo!(Naive, NalgebraBackend, backend, points, amps, size)
            .with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaiveDefaultConstraint() -> EmissionConstraintPtr {
    GS::new(NalgebraBackend::new().unwrap()).constraint().into()
}
