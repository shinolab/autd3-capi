use autd3capi_driver::LinkPtr;

use crate::controller::ControllerPtr;

pub mod audit;
pub mod nop;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkGet(cnt: ControllerPtr) -> LinkPtr {
    LinkPtr(&cnt.inner.link as *const _ as _)
}
