use autd3capi_driver::{
    ConstPtr, DatagramPtr, DebugTypeWrap, GeometryPtr,
    autd3::{core::datagram::GPIOOut, driver::geometry::Device},
    driver::{datagram::GPIOOutputs, firmware::fpga::DebugType},
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
            unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u16, GPIOOut, *mut DebugTypeWrap),
        >(f);
        GPIOOutputs::<Box<dyn Fn(&Device, GPIOOut) -> DebugType + Send + Sync>>::new(Box::new(
            move |dev: &Device, gpio: GPIOOut| {
                let mut debug_type = DebugTypeWrap::default();
                f(
                    context,
                    geometry,
                    dev.idx() as _,
                    gpio,
                    &mut debug_type as *mut _,
                );
                debug_type.convert(dev)
            },
        ))
        .into()
    }
}
