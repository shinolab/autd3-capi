#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintPtr};
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
pub unsafe extern "C" fn AUTDGainHoloGSPATSphere(
    backend: BackendPtr,
    points: *const f64,
    amps: *const f64,
    size: u64,
    repeat: u32,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    create_holo!(GSPAT, NalgebraBackend, Sphere, backend, points, amps, size)
        .with_repeat(repeat as _)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATT4010A1(
    backend: BackendPtr,
    points: *const f64,
    amps: *const f64,
    size: u64,
    repeat: u32,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    create_holo!(GSPAT, NalgebraBackend, T4010A1, backend, points, amps, size)
        .with_repeat(repeat as _)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGSPATIsDefault(gs: GainPtr) -> bool {
    let g = take_gain!(gs, GSPAT<Sphere,NalgebraBackend<Sphere>>);
    let default = GSPAT::new(std::sync::Arc::new(NalgebraBackend::default()));
    g.constraint() == default.constraint() && g.repeat() == default.repeat()
}
