#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintWrap};
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
pub unsafe extern "C" fn AUTDGainHoloNaiveSphere(
    backend: BackendPtr,
    points: *const f32,
    amps: *const f32,
    size: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(Naive, NalgebraBackend, Sphere, backend, points, amps, size)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaiveT4010A1(
    backend: BackendPtr,
    points: *const f32,
    amps: *const f32,
    size: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(Naive, NalgebraBackend, T4010A1, backend, points, amps, size)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainNaiveIsDefault(gs: GainPtr) -> bool {
    let g = take_gain!(gs, Naive<Sphere,NalgebraBackend<Sphere>>);
    let default = Naive::new(std::sync::Arc::new(NalgebraBackend::default()), []);
    g.constraint() == default.constraint()
}
