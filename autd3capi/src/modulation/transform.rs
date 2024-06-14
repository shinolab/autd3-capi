use autd3capi_driver::{driver::datagram::ModulationTransform, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithTransform(
    m: ModulationPtr,
    f: ConstPtr,
    context: ConstPtr,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    ModulationTransform::new(*take!(m, Box<M>), move |i, d| {
        std::mem::transmute::<*const std::ffi::c_void, unsafe extern "C" fn(ConstPtr, u16, u8) -> u8>(f)(
            context, i as _, d,
        )
    })
    .with_loop_behavior(loop_behavior.into())
    .into()
}
