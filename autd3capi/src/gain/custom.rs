use autd3capi_driver::{autd3::gain::Custom, *};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContextPtr(pub ConstPtr);

unsafe impl Send for ContextPtr {}
unsafe impl Sync for ContextPtr {}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCustom(
    f: ConstPtr,
    context: ContextPtr,
    geometry: GeometryPtr,
) -> GainPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ContextPtr, GeometryPtr, u32, u8, *mut Drive),
    >(f);
    Custom::new(move |dev| {
        let dev_idx = dev.idx() as u32;
        move |tr| {
            let mut d = driver::firmware::fpga::Drive::null();
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
