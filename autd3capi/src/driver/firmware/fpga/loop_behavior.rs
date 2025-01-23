use std::num::NonZeroU16;

use autd3capi_driver::autd3::prelude::LoopBehavior;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorInfinite() -> LoopBehavior {
    LoopBehavior::Infinite
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorFinite(v: u16) -> LoopBehavior {
    LoopBehavior::Finite(NonZeroU16::new(v).unwrap())
}
