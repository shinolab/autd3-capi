use autd3capi_driver::driver::firmware::fpga::LoopBehavior as RawLoopBehavior;
use autd3capi_driver::LoopBehavior;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorInfinite() -> LoopBehavior {
    RawLoopBehavior::infinite().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorFinite(v: u32) -> LoopBehavior {
    RawLoopBehavior::finite(v).unwrap().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorOnce() -> LoopBehavior {
    RawLoopBehavior::once().into()
}
