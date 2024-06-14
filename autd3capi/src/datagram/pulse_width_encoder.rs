use autd3capi_driver::{
    autd3::derive::Device, driver::datagram::PulseWidthEncoder, ConstPtr, ContextPtr, DatagramPtr,
    GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoder(
    f: ConstPtr,
    context: ContextPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        *const std::ffi::c_void,
        unsafe extern "C" fn(ContextPtr, GeometryPtr, u16, u16) -> u16,
    >(f);
    PulseWidthEncoder::<
        Box<dyn Fn(usize) -> u16 + Send + Sync>,
        Box<dyn Fn(&Device) -> Box<dyn Fn(usize) -> u16 + Send + Sync>>,
    >::new(Box::new(move |dev: &Device| {
        let dev_idx = dev.idx() as _;
        Box::new(move |i| f(context, geometry, dev_idx, i as u16))
    }))
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoderDefault() -> DatagramPtr {
    PulseWidthEncoder::default().into()
}
