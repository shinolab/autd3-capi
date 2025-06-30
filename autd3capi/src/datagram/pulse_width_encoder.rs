use autd3capi_driver::{
    ConstPtr, DatagramPtr, GeometryPtr,
    autd3::{core::gain::Intensity, driver::geometry::Device, prelude::PulseWidth},
    driver::datagram::PulseWidthEncoder,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoder256(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    unsafe {
        let f = std::mem::transmute::<
            ConstPtr,
            unsafe extern "C" fn(ConstPtr, GeometryPtr, u16, u8) -> u8,
        >(f);
        PulseWidthEncoder::<
            PulseWidth<8, u8>,
            Box<dyn Fn(&Device) -> Box<dyn Fn(Intensity) -> PulseWidth<8, u8> + Send + Sync>>,
        >::new(Box::new(move |dev: &Device| {
            let dev_idx = dev.idx() as _;
            Box::new(move |i| PulseWidth::new(f(context, geometry, dev_idx, i.0)).unwrap())
        }))
        .into()
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoder256Default() -> DatagramPtr {
    PulseWidthEncoder::<PulseWidth<8, u8>, _>::default().into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoder512(
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
            PulseWidth<9, u16>,
            Box<dyn Fn(&Device) -> Box<dyn Fn(Intensity) -> PulseWidth<9, u16> + Send + Sync>>,
        >::new(Box::new(move |dev: &Device| {
            let dev_idx = dev.idx() as _;
            Box::new(move |i| PulseWidth::new(f(context, geometry, dev_idx, i.0)).unwrap())
        }))
        .into()
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPulseWidthEncoder512Default() -> DatagramPtr {
    PulseWidthEncoder::<PulseWidth<9, u16>, _>::default().into()
}
