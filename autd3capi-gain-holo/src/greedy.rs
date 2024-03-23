#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, EmissionConstraintPtr};
use autd3_gain_holo::*;
use autd3capi_def::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedy(
    points: *const f64,
    amps: *const f64,
    size: u64,
    div: u8,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    create_holo!(Greedy, points, amps, size)
        .with_phase_div(div)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGreedyIsDefault(greedy: GainPtr) -> bool {
    let g = take_gain!(greedy, Greedy);
    let default = Greedy::new();
    g.constraint() == default.constraint() && g.phase_div() == default.phase_div()
}
