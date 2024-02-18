use autd3capi_def::{driver::datagram::Clear, DatagramPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramClear() -> DatagramPtr {
    Clear::new().into()
}
