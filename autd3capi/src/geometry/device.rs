#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    driver::geometry::{Quaternion, UnitQuaternion, Vector3},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDevice(geo: GeometryPtr, dev_idx: u16) -> DevicePtr {
    DevicePtr(&raw const geo[dev_idx as usize] as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceNumTransducers(dev: DevicePtr) -> u32 {
    dev.num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceGetSoundSpeed(dev: DevicePtr) -> f32 {
    dev.sound_speed
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceSetSoundSpeed(mut geo: GeometryPtr, dev_idx: u16, value: f32) {
    geo[dev_idx as usize].sound_speed = value;
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceSetSoundSpeedFromTemp(
    mut geo: GeometryPtr,
    dev_idx: u16,
    temp: f32,
    k: f32,
    r: f32,
    m: f32,
) {
    geo[dev_idx as usize].set_sound_speed_from_temp_with(temp, k, r, m);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceCenter(dev: DevicePtr) -> Vector3 {
    dev.center()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceTranslate(mut geo: GeometryPtr, dev_idx: u16, t: Vector3) {
    geo[dev_idx as usize].translate(t);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceRotate(mut geo: GeometryPtr, dev_idx: u16, r: Quaternion) {
    geo[dev_idx as usize].rotate(UnitQuaternion::from_quaternion(r));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceAffine(
    mut geo: GeometryPtr,
    dev_idx: u16,
    t: Vector3,
    r: Quaternion,
) {
    geo[dev_idx as usize].affine(t, UnitQuaternion::from_quaternion(r));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceEnableSet(mut geo: GeometryPtr, dev_idx: u16, value: bool) {
    geo[dev_idx as usize].enable = value;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceEnableGet(dev: DevicePtr) -> bool {
    dev.enable
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceWavelength(dev: DevicePtr) -> f32 {
    dev.wavelength()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceWavenumber(dev: DevicePtr) -> f32 {
    dev.wavenumber()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceRotation(dev: DevicePtr) -> Quaternion {
    *dev.rotation().quaternion()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceDirectionX(dev: DevicePtr) -> Vector3 {
    *dev.x_direction()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceDirectionY(dev: DevicePtr) -> Vector3 {
    *dev.y_direction()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceDirectionAxial(dev: DevicePtr) -> Vector3 {
    *dev.axial_direction()
}
