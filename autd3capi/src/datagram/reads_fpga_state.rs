use autd3capi_driver::{
    autd3::derive::Device, driver::datagram::ReadsFPGAState, ConstPtr, DatagramPtr, GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramReadsFPGAState(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        ConstPtr,
        unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u16) -> bool,
    >(f);
    ReadsFPGAState::<Box<dyn Fn(&Device) -> bool>>::new(Box::new(move |dev| {
        f(context, geometry, dev.idx() as _)
    }))
    .into()
}
