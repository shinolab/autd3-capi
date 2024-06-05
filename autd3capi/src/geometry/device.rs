#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    driver::geometry::{Quaternion, UnitQuaternion, Vector3},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDevice(geo: GeometryPtr, dev_idx: u16) -> DevicePtr {
    DevicePtr(&geo[dev_idx as usize] as *const _ as _)
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
pub unsafe extern "C" fn AUTDDeviceSetSoundSpeed(mut dev: DevicePtr, value: f32) {
    dev.sound_speed = value;
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceSetSoundSpeedFromTemp(
    mut dev: DevicePtr,
    temp: f32,
    k: f32,
    r: f32,
    m: f32,
) {
    dev.set_sound_speed_from_temp_with(temp, k, r, m);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceCenter(dev: DevicePtr, center: *mut f32) {
    let c = dev.center();
    center.add(0).write(c.x);
    center.add(1).write(c.y);
    center.add(2).write(c.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceTranslate(mut dev: DevicePtr, x: f32, y: f32, z: f32) {
    dev.translate(Vector3::new(x, y, z));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceRotate(mut dev: DevicePtr, w: f32, i: f32, j: f32, k: f32) {
    dev.rotate(UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceAffine(
    mut dev: DevicePtr,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    i: f32,
    j: f32,
    k: f32,
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
pub unsafe extern "C" fn AUTDDeviceWavelength(dev: DevicePtr) -> f32 {
    dev.wavelength()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceWavenumber(dev: DevicePtr) -> f32 {
    dev.wavenumber()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceRotation(dev: DevicePtr, rot: *mut f32) {
    let r = dev.rotation();
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceDirectionX(dev: DevicePtr, dir: *mut f32) {
    let d = dev.x_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceDirectionY(dev: DevicePtr, dir: *mut f32) {
    let d = dev.y_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceDirectionAxial(dev: DevicePtr, dir: *mut f32) {
    let d = dev.axial_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}
