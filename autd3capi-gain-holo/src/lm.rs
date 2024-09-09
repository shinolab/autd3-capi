#![allow(clippy::missing_safety_doc)]

use std::num::NonZeroUsize;

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
pub unsafe extern "C" fn AUTDGainHoloLMSphere(
    backend: BackendPtr,
    points: *const Vector3,
    amps: *const f32,
    size: u32,
    eps_1: f32,
    eps_2: f32,
    tau: f32,
    k_max_nonzero: u32,
    initial_ptr: *const f32,
    initial_len: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(LM, NalgebraBackend, Sphere, backend, points, amps, size)
        .with_eps_1(eps_1)
        .with_eps_2(eps_2)
        .with_tau(tau)
        .with_k_max(NonZeroUsize::new_unchecked(k_max_nonzero as _))
        .with_initial(vec_from_raw!(initial_ptr, f32, initial_len))
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMT4010A1(
    backend: BackendPtr,
    points: *const Vector3,
    amps: *const f32,
    size: u32,
    eps_1: f32,
    eps_2: f32,
    tau: f32,
    k_max_nonzero: u32,
    initial_ptr: *const f32,
    initial_len: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(LM, NalgebraBackend, T4010A1, backend, points, amps, size)
        .with_eps_1(eps_1)
        .with_eps_2(eps_2)
        .with_tau(tau)
        .with_k_max(NonZeroUsize::new_unchecked(k_max_nonzero as _))
        .with_initial(vec_from_raw!(initial_ptr, f32, initial_len))
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainLMIsDefault(gs: GainPtr) -> bool {
    let g = take_gain!(gs, LM<Sphere,NalgebraBackend<Sphere>>);
    let default = LM::new(NalgebraBackend::<Sphere>::new().unwrap(), []);
    g.constraint() == default.constraint()
        && g.eps_1() == default.eps_1()
        && g.eps_2() == default.eps_2()
        && g.tau() == default.tau()
        && g.k_max() == default.k_max()
        && g.initial() == default.initial()
}
