use autd3capi_driver::{DatagramPtr, driver::datagram::Clear};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramClear() -> DatagramPtr {
    Clear::new().into()
}
