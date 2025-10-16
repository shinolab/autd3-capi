#![allow(clippy::missing_safety_doc)]

pub mod device;
pub mod rotation;
pub mod transducer;

use autd3capi_driver::*;
use driver::{
    autd3_device::AUTD3,
    geometry::{Quaternion, UnitQuaternion},
};

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
pub unsafe extern "C" fn AUTDGeometryCenter(geo: GeometryPtr) -> Point3 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry() {
        unsafe {
            let cnt = crate::tests::create_controller();

            let geo = AUTDGeometry(cnt);
            assert_eq!(AUTDGeometryNumDevices(geo), 1);
            assert_eq!(AUTDGeometryNumTransducers(geo), 249);
        }
    }
}
