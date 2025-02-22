use autd3capi_driver::{ControllerPtr, LinkPtr};

pub mod audit;
pub mod nop;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkGet(cnt: ControllerPtr) -> LinkPtr {
    LinkPtr(cnt.link() as *const _ as _)
}
