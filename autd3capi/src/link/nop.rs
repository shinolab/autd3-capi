#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{LinkPtr, autd3::link::Nop};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkNop() -> LinkPtr {
    Nop::new().into()
}
