#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorInfinite() -> LoopBehavior {
    autd3_driver::derive::LoopBehavior::Infinite.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorFinite(v: u32) -> LoopBehavior {
    autd3_driver::derive::LoopBehavior::Finite(NonZeroU32::new(v).unwrap()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorOnce() -> LoopBehavior {
    autd3_driver::derive::LoopBehavior::once().into()
}
