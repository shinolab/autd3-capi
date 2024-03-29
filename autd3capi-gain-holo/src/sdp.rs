#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintPtr};
use autd3_gain_holo::*;
use autd3capi_def::{driver::geometry::Vector3, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDP(
    backend: BackendPtr,
    points: *const f64,
    amps: *const f64,
    size: u64,
    alpha: f64,
    lambda: f64,
    repeat: u32,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    create_holo!(SDP, NalgebraBackend, backend, points, amps, size)
        .with_alpha(alpha)
        .with_lambda(lambda)
        .with_repeat(repeat as _)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSDPIsDefault(gs: GainPtr) -> bool {
    let g = take_gain!(gs, SDP<NalgebraBackend>);
    let default = SDP::new(NalgebraBackend::new().unwrap());
    g.constraint() == default.constraint()
        && g.alpha() == default.alpha()
        && g.lambda() == default.lambda()
        && g.repeat() == default.repeat()
}
