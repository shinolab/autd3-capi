#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::*;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransducer(dev: DevicePtr, idx: u8) -> TransducerPtr {
    TransducerPtr(&raw const dev[idx as usize] as _)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDTransducerPosition(tr: TransducerPtr) -> Point3 {
    tr.position()
}
