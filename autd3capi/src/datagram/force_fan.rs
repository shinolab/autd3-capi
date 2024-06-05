use autd3capi_driver::{
    autd3::derive::Device, driver::datagram::ForceFan, ConstPtr, ContextPtr, DatagramPtr,
    GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramForceFan(
    f: ConstPtr,
    context: ContextPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<_, unsafe extern "C" fn(ContextPtr, GeometryPtr, u16) -> bool>(f);
    ForceFan::new(Box::new(move |dev: &Device| f(context, geometry, dev.idx() as _)) as Box<_>)
        .into()
}
