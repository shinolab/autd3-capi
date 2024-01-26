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
    create_holo!(Naive, NalgebraBackend, backend, points, amps, size)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainNaiveIsDefault(gs: GainPtr) -> bool {
    let g = take_gain!(gs, Naive<NalgebraBackend>);
    let default = Naive::new(NalgebraBackend::new().unwrap());
    g.constraint() == default.constraint()
}
