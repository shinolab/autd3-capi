#![allow(clippy::missing_safety_doc)]

pub mod device;
pub mod rotation;
pub mod transducer;

use autd3capi_driver::*;

use crate::controller::ControllerPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometry(cnt: ControllerPtr) -> GeometryPtr {
    GeometryPtr(&cnt.inner.geometry as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryNumDevices(geo: GeometryPtr) -> u32 {
    geo.num_devices() as _
}
