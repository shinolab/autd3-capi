use autd3capi_driver::{driver::datagram::GainTransform, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainWithTransform(
    g: GainPtr,
    f: ConstPtr,
    context: ContextPtr,
    geometry: GeometryPtr,
) -> GainPtr {
    let f = std::mem::transmute::<
        *const std::ffi::c_void,
        unsafe extern "C" fn(
            ContextPtr,
            GeometryPtr,
            u32,
            u8,
            driver::firmware::fpga::Drive,
            *mut driver::firmware::fpga::Drive,
        ),
    >(f);
    GainTransform::new(*take!(g, Box<G>), move |dev| {
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
