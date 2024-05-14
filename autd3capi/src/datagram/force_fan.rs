use autd3capi_def::{ConstPtr, DatagramPtr, DynamicConfigureForceFan, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigureForceFan(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<_, unsafe extern "C" fn(ConstPtr, GeometryPtr, u32) -> bool>(f);
    DynamicConfigureForceFan::new(
        geometry
            .devices()
            .map(move |dev| (dev.idx(), f(context, geometry, dev.idx() as u32)))
            .collect(),
    )
    .into()
}
