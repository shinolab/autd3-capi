#![allow(clippy::missing_safety_doc)]

use autd3_backend_cuda::*;
use autd3_gain_holo::*;
use autd3capi_driver::{driver::acoustics::directivity::Sphere, *};
use autd3capi_gain_holo::{constraint::EmissionConstraintWrap, BackendPtr, ResultBackend};

#[macro_export]
macro_rules! create_holo {
    ($type:tt, $backend_type:tt, $direcivity:tt, $backend:expr, $points:expr, $amps:expr, $size:expr) => {
        $type::<$direcivity, $backend_type>::new(
            ($backend.0 as *const std::sync::Arc<$backend_type>)
                .as_ref()
                .unwrap()
                .clone(),
            (0..$size as usize).map(|i| {
                let p = $points.add(i).read();
                let amp = $amps.add(i).read() * Pa;
                (p, amp)
            }),
        )
    };
}

#[no_mangle]
pub unsafe extern "C" fn AUTDCUDASetUltrasoundFreq(f: u32) {
    autd3capi_driver::driver::set_ultrasound_freq(f * autd3capi_driver::driver::defined::Hz);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCUDABackend() -> ResultBackend {
    match CUDABackend::new() {
        Ok(b) => ResultBackend {
            result: BackendPtr(Box::into_raw(Box::new(b)) as _),
            err_len: 0,
            err: ConstPtr(std::ptr::null_mut()),
        },
        Err(e) => {
            let err = e.to_string();
            ResultBackend {
                result: BackendPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
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
    points: *const Vector3,
    amps: *const f32,
    size: u32,
    alpha: f32,
    lambda: f32,
    repeat: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(SDP, CUDABackend, Sphere, backend, points, amps, size)
        .with_alpha(alpha)
        .with_lambda(lambda)
        .with_repeat(repeat as _)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGS(
    backend: BackendPtr,
    points: *const Vector3,
    amps: *const f32,
    size: u32,
    repeat: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(GS, CUDABackend, Sphere, backend, points, amps, size)
        .with_repeat(repeat as _)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGSPAT(
    backend: BackendPtr,
    points: *const Vector3,
    amps: *const f32,
    size: u32,
    repeat: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(GSPAT, CUDABackend, Sphere, backend, points, amps, size)
        .with_repeat(repeat as _)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDANaive(
    backend: BackendPtr,
    points: *const Vector3,
    amps: *const f32,
    size: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(Naive, CUDABackend, Sphere, backend, points, amps, size)
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDALM(
    backend: BackendPtr,
    points: *const Vector3,
    amps: *const f32,
    size: u32,
    eps_1: f32,
    eps_2: f32,
    tau: f32,
    k_max: u32,
    constraint: EmissionConstraintWrap,
    initial_ptr: *const f32,
    initial_len: u64,
) -> GainPtr {
    create_holo!(LM, CUDABackend, Sphere, backend, points, amps, size)
        .with_eps_1(eps_1)
        .with_eps_2(eps_2)
        .with_tau(tau)
        .with_k_max(k_max as _)
        .with_initial(vec_from_raw!(initial_ptr, f32, initial_len))
        .with_constraint(constraint.into())
        .into()
}
