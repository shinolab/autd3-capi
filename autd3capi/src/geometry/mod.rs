#![allow(clippy::missing_safety_doc)]

pub mod device;
pub mod rotation;
pub mod transducer;

use autd3capi_driver::*;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGeometry(cnt: ControllerPtr) -> GeometryPtr {
    GeometryPtr(cnt.geometry() as *const _ as _)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryNumDevices(geo: GeometryPtr) -> u32 {
    geo.num_devices() as _
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryNumTransducers(geo: GeometryPtr) -> u32 {
    geo.num_transducers() as _
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGeometrCenter(geo: GeometryPtr) -> Point3 {
    geo.center()
}
