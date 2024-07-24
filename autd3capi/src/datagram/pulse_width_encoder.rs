use autd3capi_driver::{
    autd3::derive::Device, driver::datagram::PulseWidthEncoder, ConstPtr, DatagramPtr, GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoder(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        ConstPtr,
        unsafe extern "C" fn(ConstPtr, GeometryPtr, u16, u8) -> u8,
    >(f);
    PulseWidthEncoder::<
        Box<dyn Fn(u8) -> u8 + Send + Sync>,
        Box<dyn Fn(&Device) -> Box<dyn Fn(u8) -> u8 + Send + Sync>>,
    >::new(Box::new(move |dev: &Device| {
        let dev_idx = dev.idx() as _;
        Box::new(move |i| f(context, geometry, dev_idx, i))
    }))
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoderDefault() -> DatagramPtr {
    PulseWidthEncoder::default().into()
}
