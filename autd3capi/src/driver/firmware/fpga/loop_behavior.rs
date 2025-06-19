use std::num::NonZeroU16;

use autd3capi_driver::{LoopBehavior, autd3};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorInfinite() -> LoopBehavior {
    autd3::core::datagram::LoopBehavior::Infinite.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLoopBehaviorFinite(v: u16) -> LoopBehavior {
    autd3::core::datagram::LoopBehavior::Finite(NonZeroU16::new(v).unwrap()).into()
}
