use autd3capi_driver::{ConstPtr, DatagramPtr, DynamicReadsFPGAState, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramReadsFPGAState(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u32) -> bool,
    >(f);
    DynamicReadsFPGAState::new(
        geometry
            .devices()
            .map(move |dev| (dev.idx(), f(context, geometry, dev.idx() as u32)))
            .collect(),
    )
    .into()
}
