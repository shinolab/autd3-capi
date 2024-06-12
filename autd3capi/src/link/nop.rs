#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{autd3::link::Nop, DynamicLinkBuilder, LinkBuilderPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkNop() -> LinkBuilderPtr {
    DynamicLinkBuilder::new(Nop::builder())
}
