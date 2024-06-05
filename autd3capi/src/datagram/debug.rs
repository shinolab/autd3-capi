use autd3capi_driver::{
    autd3::derive::Device,
    driver::{datagram::DebugSettings, firmware::fpga::DebugType},
    ConstPtr, ContextPtr, DatagramPtr, DebugTypeWrap, GPIOOut, GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramDebugSettings(
    f: ConstPtr,
    context: ContextPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ContextPtr, geometry: GeometryPtr, u16, GPIOOut, *mut DebugTypeWrap),
    >(f);
    DebugSettings::<
        Box<dyn Fn(&Device, autd3capi_driver::autd3::prelude::GPIOOut) -> DebugType + Send + Sync>,
    >::new(Box::new(
        move |dev: &Device, gpio: autd3capi_driver::autd3::prelude::GPIOOut| {
            let mut debug_type = DebugTypeWrap::default();
            f(
                context,
                geometry,
                dev.idx() as _,
                gpio.into(),
                &mut debug_type as *mut _,
            );
            debug_type.convert(dev)
        },
    ))
    .into()
}
