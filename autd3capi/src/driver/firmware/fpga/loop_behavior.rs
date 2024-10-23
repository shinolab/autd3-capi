use autd3capi_driver::autd3::derive::LoopBehavior;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorInfinite() -> LoopBehavior {
    LoopBehavior::infinite()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorFinite(v: u16) -> LoopBehavior {
    LoopBehavior::finite(v).unwrap()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorOnce() -> LoopBehavior {
    LoopBehavior::once()
}
