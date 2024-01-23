#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintPtr};
use autd3_gain_holo::*;
use autd3capi_def::{driver::geometry::Vector3, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLM(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    eps_1: float,
    eps_2: float,
    tau: float,
    k_max: u32,
    initial_ptr: *const float,
    initial_len: u64,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    let mut initial = vec![0.; initial_len as usize];
    std::ptr::copy_nonoverlapping(initial_ptr, initial.as_mut_ptr(), initial_len as usize);
    GainPtr::new(
        create_holo!(LM, NalgebraBackend, backend, points, amps, size)
            .with_eps_1(eps_1)
            .with_eps_2(eps_2)
            .with_tau(tau)
            .with_k_max(k_max as _)
            .with_initial(initial)
            .with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMDefaultConstraint() -> EmissionConstraintPtr {
    LM::new(NalgebraBackend::new().unwrap()).constraint().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMDefaultEps1() -> float {
    LM::new(NalgebraBackend::new().unwrap()).eps_1()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMDefaultEps2() -> float {
    LM::new(NalgebraBackend::new().unwrap()).eps_2()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMDefaultTau() -> float {
    LM::new(NalgebraBackend::new().unwrap()).tau()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMDefaultKMax() -> u32 {
    LM::new(NalgebraBackend::new().unwrap()).k_max() as _
}
