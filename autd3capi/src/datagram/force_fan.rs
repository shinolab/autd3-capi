use autd3capi_driver::{
    autd3::derive::Device, driver::datagram::ForceFan, ConstPtr, DatagramPtr, GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramForceFan(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<ConstPtr, unsafe extern "C" fn(ConstPtr, GeometryPtr, u16) -> bool>(
        f,
    );
    ForceFan::new(Box::new(move |dev: &Device| f(context, geometry, dev.idx() as _)) as Box<_>)
        .into()
}
