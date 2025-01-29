#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{autd3::link::Nop, LinkPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkNop() -> LinkPtr {
    Nop::new().into()
}
