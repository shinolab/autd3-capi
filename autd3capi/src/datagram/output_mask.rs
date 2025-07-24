use autd3capi_driver::{
    ConstPtr, DatagramPtr, GeometryPtr,
    autd3::driver::geometry::Device,
    core::datagram::Segment,
    driver::{datagram::OutputMask, geometry::Transducer},
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramOutputMaskWithSegment(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
    segment: Segment,
) -> DatagramPtr {
    unsafe {
        let f = std::mem::transmute::<
            ConstPtr,
            unsafe extern "C" fn(ConstPtr, GeometryPtr, u16, u8) -> bool,
        >(f);
        OutputMask::with_segment(
            Box::new(move |dev: &Device| {
                let dev_idx = dev.idx() as _;
                Box::new(move |tr: &Transducer| {
                    let tr_idx = tr.idx() as _;
                    f(context, geometry, dev_idx, tr_idx)
                }) as Box<dyn Fn(&Transducer) -> bool + Send + Sync>
            })
                as Box<dyn Fn(&Device) -> Box<dyn Fn(&Transducer) -> bool + Send + Sync>>,
            segment,
        )
        .into()
    }
}
