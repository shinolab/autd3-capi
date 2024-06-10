#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransducer(dev: DevicePtr, idx: u8) -> TransducerPtr {
    TransducerPtr(&dev[idx as usize] as *const _ as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerPosition(tr: TransducerPtr) -> Vector3 {
    *tr.position()
}
