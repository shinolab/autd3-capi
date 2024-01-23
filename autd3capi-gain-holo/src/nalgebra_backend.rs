#![allow(clippy::missing_safety_doc)]

use crate::BackendPtr;
use autd3_gain_holo::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNalgebraBackend() -> BackendPtr {
    BackendPtr(Box::into_raw(Box::new(NalgebraBackend::new().unwrap())) as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteNalgebraBackend(backend: BackendPtr) {
    let _ = Box::from_raw(backend.0 as *mut std::sync::Arc<NalgebraBackend>);
}
