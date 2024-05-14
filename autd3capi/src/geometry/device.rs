#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    driver::geometry::{Quaternion, UnitQuaternion, Vector3},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDevice(geo: GeometryPtr, dev_idx: u32) -> DevicePtr {
    DevicePtr(&geo[dev_idx as usize] as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceNumTransducers(dev: DevicePtr) -> u32 {
    dev.num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceGetSoundSpeed(dev: DevicePtr) -> f64 {
    dev.sound_speed
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceSetSoundSpeed(mut dev: DevicePtr, value: f64) {
    dev.sound_speed = value;
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceSetSoundSpeedFromTemp(
    mut dev: DevicePtr,
    temp: f64,
    k: f64,
    r: f64,
    m: f64,
) {
    dev.set_sound_speed_from_temp_with(temp, k, r, m);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceGetAttenuation(dev: DevicePtr) -> f64 {
    dev.attenuation
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceSetAttenuation(mut dev: DevicePtr, value: f64) {
    dev.attenuation = value;
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceCenter(dev: DevicePtr, center: *mut f64) {
    let c = dev.center();
    center.add(0).write(c.x);
    center.add(1).write(c.y);
    center.add(2).write(c.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceTranslate(mut dev: DevicePtr, x: f64, y: f64, z: f64) {
    dev.translate(Vector3::new(x, y, z));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceRotate(mut dev: DevicePtr, w: f64, i: f64, j: f64, k: f64) {
    dev.rotate(UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceAffine(
    mut dev: DevicePtr,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    i: f64,
    j: f64,
    k: f64,
) {
    dev.affine(
        Vector3::new(x, y, z),
        UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)),
    );
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceEnableSet(mut dev: DevicePtr, value: bool) {
    dev.enable = value;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceEnableGet(dev: DevicePtr) -> bool {
    dev.enable
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceWavelength(dev: DevicePtr) -> f64 {
    dev.wavelength()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceWavenumber(dev: DevicePtr) -> f64 {
    dev.wavenumber()
}
