#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    driver::geometry::{Quaternion, Vector3},
    *,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDevice(geo: GeometryPtr, dev_idx: u16) -> DevicePtr {
    DevicePtr(&raw const geo[dev_idx as usize] as _)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceNumTransducers(dev: DevicePtr) -> u32 {
    dev.num_transducers() as _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDDeviceCenter(dev: DevicePtr) -> Point3 {
    *dev.center()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDDeviceRotation(dev: DevicePtr) -> Quaternion {
    *dev.rotation().quaternion()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDDeviceDirectionX(dev: DevicePtr) -> Vector3 {
    dev.x_direction().into_inner()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDDeviceDirectionY(dev: DevicePtr) -> Vector3 {
    dev.y_direction().into_inner()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDDeviceDirectionAxial(dev: DevicePtr) -> Vector3 {
    dev.axial_direction().into_inner()
}
