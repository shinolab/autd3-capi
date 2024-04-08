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

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigureDebugSettings2(
    types: *const u8,
    values: *const u16,
    geometry: GeometryPtr,
) -> DatagramPtr {
    DynamicConfigureDebugSettings::new(
        geometry
            .devices()
            .map(|dev| {
                (
                    dev.idx(),
                    DebugSettings {
                        ty: [
                            types.add(4 * dev.idx()).read(),
                            types.add(4 * dev.idx() + 1).read(),
                            types.add(4 * dev.idx() + 2).read(),
                            types.add(4 * dev.idx() + 3).read(),
                        ],
                        value: [
                            values.add(4 * dev.idx()).read(),
                            values.add(4 * dev.idx() + 1).read(),
                            values.add(4 * dev.idx() + 2).read(),
                            values.add(4 * dev.idx() + 3).read(),
                        ],
                    },
                )
            })
            .collect(),
    )
    .into()
}
