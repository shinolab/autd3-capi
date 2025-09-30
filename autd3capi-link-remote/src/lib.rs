#![allow(clippy::missing_safety_doc)]

use std::{ffi::c_char, net::SocketAddr};

use autd3capi_driver::*;

use autd3_link_remote::*;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemote(addr: *const c_char) -> ResultLink {
    let addr = if addr.is_null() {
        ""
    } else {
        validate_cstr!(addr, LinkPtr, ResultLink)
    };
    addr.parse::<SocketAddr>().map(Remote::new).into()
}
