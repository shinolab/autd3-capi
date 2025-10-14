#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::*;

use autd3_link_twincat::*;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCAT() -> ResultLink {
    TwinCAT::new().into()
}
