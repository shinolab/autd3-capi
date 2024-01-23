#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, EmissionConstraintPtr};
use autd3_gain_holo::*;
use autd3capi_def::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedy(
    points: *const float,
    amps: *const float,
    size: u64,
    div: u8,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        create_holo!(Greedy, points, amps, size)
            .with_phase_div(div)
            .with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyDefaultConstraint() -> EmissionConstraintPtr {
    Greedy::new().constraint().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyDefaultPhaseDiv() -> u8 {
    Greedy::new().phase_div()
}
