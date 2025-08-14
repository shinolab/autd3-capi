use autd3capi_driver::{
    ConstPtr, DatagramPtr, GPIOOutputTypeWrap, GeometryPtr,
    autd3::{core::firmware::GPIOOut, driver::geometry::Device},
    driver::{datagram::GPIOOutputType, datagram::GPIOOutputs},
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramGPIOOutputs(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    unsafe {
        let f = std::mem::transmute::<
            ConstPtr,
            unsafe extern "C" fn(
                ConstPtr,
                geometry: GeometryPtr,
                u16,
                GPIOOut,
                *mut GPIOOutputTypeWrap,
            ),
        >(f);
        GPIOOutputs::<Box<dyn Fn(&Device, GPIOOut) -> Option<GPIOOutputType> + Send + Sync>>::new(
            Box::new(move |dev: &Device, gpio: GPIOOut| {
                let mut debug_type = GPIOOutputTypeWrap::default();
                f(
                    context,
                    geometry,
                    dev.idx() as _,
                    gpio,
                    &mut debug_type as *mut _,
                );
                debug_type.convert(dev)
            }),
        )
        .into()
    }
}
