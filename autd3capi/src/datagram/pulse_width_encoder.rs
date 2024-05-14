use autd3capi_driver::{
    driver::datagram::PulseWidthEncoder, vec_from_raw, DatagramPtr, ResultDatagram,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoder(
    buf: *const u16,
    len: u32,
) -> ResultDatagram {
    PulseWidthEncoder::new(vec_from_raw!(buf, u16, len)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoderDefault() -> DatagramPtr {
    PulseWidthEncoder::default().into()
}
