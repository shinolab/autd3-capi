#![allow(clippy::missing_safety_doc)]

use crate::BackendPtr;
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Sphere, T4010A1},
    take,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDNalgebraBackendSphere() -> BackendPtr {
    BackendPtr(Box::into_raw(Box::new(std::sync::Arc::new(
        NalgebraBackend::<Sphere>::new(),
    ))) as _)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDNalgebraBackendT4010A1() -> BackendPtr {
    BackendPtr(Box::into_raw(Box::new(std::sync::Arc::new(
        NalgebraBackend::<T4010A1>::new(),
    ))) as _)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDDeleteNalgebraBackendSphere(backend: BackendPtr) {
    let _ = take!(backend, std::sync::Arc<NalgebraBackend<Sphere>>);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDDeleteNalgebraBackendT4010A1(backend: BackendPtr) {
    let _ = take!(backend, std::sync::Arc<NalgebraBackend<T4010A1>>);
}
