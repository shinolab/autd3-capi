use std::time::Duration;

use autd3capi_driver::{autd3::prelude::IntoDatagramWithTimeout, DatagramPtr, DynDatagram};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramWithTimeout(d: DatagramPtr, timeout_ns: i64) -> DatagramPtr {
    Box::<DynDatagram>::from(d)
        .with_timeout(if timeout_ns < 0 {
            None
        } else {
            Some(Duration::from_nanos(timeout_ns as u64))
        })
        .into()
}
