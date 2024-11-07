use autd3capi_driver::{
    autd3::prelude::IntoDatagramWithTimeout, DatagramPtr, DynDatagram, OptionDuration,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramWithTimeout(
    d: DatagramPtr,
    timeout: OptionDuration,
) -> DatagramPtr {
    Box::<DynDatagram>::from(d)
        .with_timeout(timeout.into())
        .into()
}
