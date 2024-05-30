#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransducer(dev: DevicePtr, idx: u32) -> TransducerPtr {
    TransducerPtr(&dev[idx as usize] as *const _ as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerPosition(tr: TransducerPtr, pos: *mut f64) {
    let p = tr.position();
    pos.add(0).write(p.x);
    pos.add(1).write(p.y);
    pos.add(2).write(p.z);
}
