use autd3capi_def::{ControllerPtr, LinkPtr};

pub mod audit;
pub mod nop;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkGet(cnt: ControllerPtr) -> LinkPtr {
    LinkPtr(&cnt.inner.link as *const _ as _)
}
