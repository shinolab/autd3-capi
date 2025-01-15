use autd3capi_driver::{
    autd3::{core::gain::Drive, gain::Custom},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCustom(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> GainPtr {
    let f = std::mem::transmute::<
        ConstPtr,
        unsafe extern "C" fn(ConstPtr, GeometryPtr, u16, u8, *mut Drive),
    >(f);
    Custom::new(move |dev| {
        let dev_idx = dev.idx() as _;
        move |tr| {
            let mut d = driver::firmware::fpga::Drive::NULL;
            f(
                context,
                geometry,
                dev_idx,
                tr.idx() as u8,
                &mut d as *mut _ as *mut _,
            );
            d
        }
    })
    .into()
}
