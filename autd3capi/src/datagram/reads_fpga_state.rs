use autd3capi_driver::{
    ConstPtr, DatagramPtr, GeometryPtr, autd3::driver::geometry::Device,
    driver::datagram::ReadsFPGAState,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramReadsFPGAState(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    unsafe {
        let f = std::mem::transmute::<
            ConstPtr,
            unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u16) -> bool,
        >(f);
        ReadsFPGAState::<Box<dyn Fn(&Device) -> bool>>::new(Box::new(move |dev| {
            f(context, geometry, dev.idx() as _)
        }))
        .into()
    }
}
