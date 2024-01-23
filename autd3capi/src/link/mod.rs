use autd3capi_def::{cast, Cnt, ControllerPtr, LinkPtr};

pub mod audit;
pub mod nop;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkGet(cnt: ControllerPtr) -> LinkPtr {
    LinkPtr(&cast!(cnt.0, Cnt).inner.link as *const _ as _)
}
