use std::num::NonZeroU16;

use autd3capi_driver::{autd3, LoopBehavior};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorInfinite() -> LoopBehavior {
    autd3::driver::firmware::fpga::LoopBehavior::Infinite.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorFinite(v: u16) -> LoopBehavior {
    autd3::driver::firmware::fpga::LoopBehavior::Finite(NonZeroU16::new(v).unwrap()).into()
}
