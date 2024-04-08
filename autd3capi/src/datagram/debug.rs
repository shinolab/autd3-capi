use autd3capi_def::{
    ConstPtr, DatagramPtr, DebugSettings, DynamicConfigureDebugSettings, GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigureDebugSettings(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u32) -> DebugSettings,
    >(f);
    DynamicConfigureDebugSettings::new(
        geometry
            .devices()
            .map(move |dev| (dev.idx(), f(context, geometry, dev.idx() as u32)))
            .collect(),
    )
    .into()
}
