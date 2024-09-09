use autd3capi_driver::{autd3::datagram::gain::IntoGainTransform, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainWithTransform(
    g: GainPtr,
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> GainPtr {
    let f = std::mem::transmute::<
        ConstPtr,
        unsafe extern "C" fn(
            ConstPtr,
            GeometryPtr,
            u16,
            u8,
            driver::firmware::fpga::Drive,
            *mut driver::firmware::fpga::Drive,
        ),
    >(f);
    take!(g, Box<G>)
        .with_transform(move |dev| {
            let dev_idx = dev.idx() as u16;
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
