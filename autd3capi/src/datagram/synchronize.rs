use autd3capi_def::{driver::datagram::Synchronize, DatagramPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSynchronize() -> DatagramPtr {
    Synchronize::new().into()
}
