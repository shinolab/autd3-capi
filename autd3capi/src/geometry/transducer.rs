#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransducer(
    geometry: GeometryPtr,
    dev_idx: u16,
    idx: u8,
) -> TransducerPtr {
    TransducerPtr(&geometry[dev_idx as usize][idx as usize] as *const _ as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerPosition(tr: TransducerPtr) -> Vector3 {
    *tr.position()
}
