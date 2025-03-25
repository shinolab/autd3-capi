use autd3capi_driver::{
    ConstPtr, DatagramPtr, GeometryPtr,
    autd3::{core::gain::EmitIntensity, driver::geometry::Device, prelude::PulseWidth},
    driver::datagram::PulseWidthEncoder,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoder(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    unsafe {
        let f = std::mem::transmute::<
            ConstPtr,
            unsafe extern "C" fn(ConstPtr, GeometryPtr, u16, u8) -> u16,
        >(f);
        PulseWidthEncoder::<
            Box<dyn Fn(EmitIntensity) -> PulseWidth<u16, 9> + Send + Sync>,
            Box<dyn Fn(&Device) -> Box<dyn Fn(EmitIntensity) -> PulseWidth<u16, 9> + Send + Sync>>,
        >::new(Box::new(move |dev: &Device| {
            let dev_idx = dev.idx() as _;
            Box::new(move |i| PulseWidth::new(f(context, geometry, dev_idx, i.0)).unwrap())
        }))
        .into()
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoderDefault() -> DatagramPtr {
    PulseWidthEncoder::default().into()
}
