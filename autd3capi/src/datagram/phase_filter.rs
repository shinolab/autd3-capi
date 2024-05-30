use autd3capi_driver::{
    autd3::derive::{Device, Phase, Transducer},
    driver::datagram::PhaseFilter,
    ConstPtr, ContextPtr, DatagramPtr, GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPhaseFilterAdditive(
    f: ConstPtr,
    context: ContextPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<_, unsafe extern "C" fn(ContextPtr, GeometryPtr, u32, u8) -> Phase>(
        f,
    );
    PhaseFilter::<
        Phase,
        Box<dyn Fn(&Transducer) -> Phase + Send + Sync>,
        Box<dyn Fn(&Device) -> Box<dyn Fn(&Transducer) -> Phase + Send + Sync>>,
    >::additive(Box::new(move |dev| {
        let dev_idx = dev.idx() as u32;
        Box::new(move |tr| f(context, geometry, dev_idx, tr.idx() as u8))
    }))
    .into()
}
