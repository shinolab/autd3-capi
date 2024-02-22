#![allow(clippy::missing_safety_doc)]

use autd3_backend_cuda::*;
use autd3_gain_holo::*;
use autd3capi_def::*;
use autd3capi_gain_holo::{constraint::EmissionConstraintPtr, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCUDABackend() -> ResultBackend {
    match CUDABackend::new() {
        Ok(b) => ResultBackend {
            result: BackendPtr(Box::into_raw(Box::new(b)) as _),
            err_len: 0,
            err: std::ptr::null_mut(),
        },
        Err(e) => {
            let err = e.to_string();
            ResultBackend {
                result: BackendPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDCUDABackendDelete(backend: BackendPtr) {
    let _ = take!(backend, std::sync::Arc<CUDABackend>);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDASDP(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    alpha: float,
    lambda: float,
    repeat: u32,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    create_holo!(SDP, CUDABackend, backend, points, amps, size)
        .with_alpha(alpha)
        .with_lambda(lambda)
        .with_repeat(repeat as _)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGS(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    repeat: u32,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    create_holo!(GS, CUDABackend, backend, points, amps, size)
        .with_repeat(repeat as _)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGSPAT(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    repeat: u32,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    create_holo!(GSPAT, CUDABackend, backend, points, amps, size)
        .with_repeat(repeat as _)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDANaive(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    constraint: EmissionConstraintPtr,
) -> GainPtr {
    create_holo!(Naive, CUDABackend, backend, points, amps, size)
        .with_constraint(*take!(constraint, _))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDALM(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
    eps_1: float,
    eps_2: float,
    tau: float,
    k_max: u32,
    constraint: EmissionConstraintPtr,
    initial_ptr: *const float,
    initial_len: u64,
) -> GainPtr {
    create_holo!(LM, CUDABackend, backend, points, amps, size)
        .with_eps_1(eps_1)
        .with_eps_2(eps_2)
        .with_tau(tau)
        .with_k_max(k_max as _)
        .with_initial(vec_from_raw!(initial_ptr, float, initial_len))
        .with_constraint(*take!(constraint, _))
        .into()
}
