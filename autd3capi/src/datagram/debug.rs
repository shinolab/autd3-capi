use autd3capi_driver::{
    autd3::{driver::geometry::Device, prelude::GPIOOut},
    driver::{datagram::DebugSettings, firmware::fpga::DebugType},
    ConstPtr, DatagramPtr, DebugTypeWrap, GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramDebugSettings(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        ConstPtr,
        unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u16, GPIOOut, *mut DebugTypeWrap),
    >(f);
    DebugSettings::<Box<dyn Fn(&Device, GPIOOut) -> DebugType + Send + Sync>>::new(Box::new(
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
