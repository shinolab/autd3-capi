use autd3capi_def::{autd3::gain::TransducerTest, *};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContextPtr(pub ConstPtr);

unsafe impl Send for ContextPtr {}
unsafe impl Sync for ContextPtr {}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainTransducerTest(
    f: ConstPtr,
    context: ContextPtr,
    geometry: GeometryPtr,
) -> GainPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ContextPtr, GeometryPtr, u32, u8, *mut Drive),
    >(f);
    TransducerTest::new(move |dev, tr| {
        let mut d = driver::common::Drive::null();
        f(
            context,
            geometry,
            dev.idx() as u32,
            tr.idx() as u8,
            &mut d as *mut _ as *mut _,
        );
        Some(d)
    })
    .into()
}
