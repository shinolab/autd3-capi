use autd3capi_driver::{driver::datagram::GainTransform2, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainWithTransform(
    g: GainPtr,
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> GainPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(
            ConstPtr,
            GeometryPtr,
            u32,
            u8,
            driver::firmware::fpga::Drive,
            *mut driver::firmware::fpga::Drive,
        ),
    >(f);
    GainTransform2::new(*take!(g, Box<G>), move |dev| {
        let dev_idx = dev.idx() as u32;
        move |tr, d| {
            let mut dst = driver::firmware::fpga::Drive::null();
            f(
                context,
                geometry,
                dev_idx,
                tr.idx() as u8,
                d,
                &mut dst as *mut _ as *mut _,
            );
            dst
        }
    })
    .into()
}
