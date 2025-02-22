use autd3capi_driver::{DatagramPtr, driver::datagram::Synchronize};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSynchronize() -> DatagramPtr {
    Synchronize::new().into()
}
