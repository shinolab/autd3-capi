#![allow(clippy::missing_safety_doc)]

use crate::BackendPtr;
use autd3_gain_holo::*;
use autd3capi_driver::{
    driver::acoustics::directivity::{Sphere, T4010A1},
    take,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNalgebraBackendSphere() -> BackendPtr {
    BackendPtr(Box::into_raw(Box::new(std::sync::Arc::new(NalgebraBackend::default()))) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNalgebraBackendT4010A1() -> BackendPtr {
    let backend = NalgebraBackend::default();
    let backend: NalgebraBackend<T4010A1> = std::mem::transmute(backend);
    BackendPtr(Box::into_raw(Box::new(std::sync::Arc::new(backend))) as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteNalgebraBackendSphere(backend: BackendPtr) {
    let _ = take!(backend, std::sync::Arc<NalgebraBackend<Sphere>>);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteNalgebraBackendT4010A1(backend: BackendPtr) {
    let _ = take!(backend, std::sync::Arc<NalgebraBackend<T4010A1>>);
}
