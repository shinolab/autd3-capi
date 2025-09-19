#![allow(clippy::missing_safety_doc)]

pub mod device;
pub mod rotation;
pub mod transducer;

use autd3capi_driver::{core::derive::Geometry, *};
use driver::{
    autd3_device::AUTD3,
    geometry::{Quaternion, UnitQuaternion},
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGeometry(cnt: ControllerPtr) -> GeometryPtr {
    use std::ops::Deref;
    let geometry: &Geometry = cnt.deref();
    GeometryPtr(geometry.as_ptr() as _)
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDGeometryReconfigure(
    mut geo: GeometryPtr,
    pos: *const Point3,
    rot: *const Quaternion,
) {
    geo.reconfigure(|dev| {
        let pos = unsafe { pos.add(dev.idx()).read() };
        let rot = unsafe { rot.add(dev.idx()).read() };
        AUTD3 {
            pos,
            rot: UnitQuaternion::from_quaternion(rot),
        }
    });
}
