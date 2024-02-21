use autd3capi_def::{
    autd3::derive::Phase, ConstPtr, DatagramPtr, DynamicConfigurePhaseFilter, GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigurePhaseFilter(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u32, u8) -> Phase,
    >(f);
    DynamicConfigurePhaseFilter::additive(
        geometry
            .devices()
            .map(move |dev| {
                (
                    dev.idx(),
                    dev.iter()
                        .map(move |tr| {
                            (
                                tr.idx(),
                                f(context, geometry, dev.idx() as u32, tr.idx() as u8),
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
    )
    .into()
}
