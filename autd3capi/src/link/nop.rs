#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::link::Nop, LinkBuilderPtr, SyncLinkBuilder};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkNop() -> LinkBuilderPtr {
    SyncLinkBuilder::new(Nop::builder())
}
