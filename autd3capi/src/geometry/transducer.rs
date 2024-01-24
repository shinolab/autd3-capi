#![allow(clippy::missing_safety_doc)]

use autd3capi_def::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransducer(dev: DevicePtr, idx: u32) -> TransducerPtr {
    TransducerPtr(&dev[idx as usize] as *const _ as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerPosition(tr: TransducerPtr, pos: *mut float) {
    let p = tr.position();
    pos.add(0).write(p.x);
    pos.add(1).write(p.y);
    pos.add(2).write(p.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerRotation(tr: TransducerPtr, rot: *mut float) {
    let r = tr.rotation();
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerDirectionX(tr: TransducerPtr, dir: *mut float) {
    let d = tr.x_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerDirectionY(tr: TransducerPtr, dir: *mut float) {
    let d = tr.y_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerDirectionZ(tr: TransducerPtr, dir: *mut float) {
    let d = tr.z_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransducerWavelength(tr: TransducerPtr, sound_speed: float) -> float {
    tr.wavelength(sound_speed)
}
