#![allow(clippy::missing_safety_doc)]

use crate::BackendPtr;
use autd3_gain_holo::*;
use autd3capi_driver::take;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDNalgebraBackend() -> BackendPtr {
    BackendPtr(Box::into_raw(Box::new(std::sync::Arc::new(NalgebraBackend::new()))) as _)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDDeleteNalgebraBackend(backend: BackendPtr) {
    let _ = unsafe { take!(backend, std::sync::Arc<NalgebraBackend>) };
}
