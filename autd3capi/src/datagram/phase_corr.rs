use autd3capi_driver::{
    autd3::{core::gain::Phase, driver::geometry::Device},
    driver::{datagram::PhaseCorrection, geometry::Transducer},
    ConstPtr, DatagramPtr, GeometryPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPhaseCorr(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        ConstPtr,
        unsafe extern "C" fn(ConstPtr, GeometryPtr, u16, u8) -> u8,
    >(f);
    PhaseCorrection::new(Box::new(move |dev: &Device| {
        let dev_idx = dev.idx() as _;
        Box::new(move |tr: &Transducer| {
            let tr_idx = tr.idx() as _;
            Phase(f(context, geometry, dev_idx, tr_idx))
        }) as Box<dyn Fn(&Transducer) -> Phase + Send + Sync>
    })
        as Box<dyn Fn(&Device) -> Box<dyn Fn(&Transducer) -> Phase + Send + Sync>>)
    .into()
}
