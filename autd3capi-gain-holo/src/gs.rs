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
    create_holo!(GS, NalgebraBackend, backend, points, amps, size)
        .with_repeat(repeat as _)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGSIsDefault(gs: GainPtr) -> bool {
    let g = take_gain!(gs, GS<NalgebraBackend>);
    let default = GS::new(NalgebraBackend::new().unwrap());
    g.constraint() == default.constraint() && g.repeat() == default.repeat()
}
