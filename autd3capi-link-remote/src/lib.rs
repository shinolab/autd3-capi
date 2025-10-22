#![allow(clippy::missing_safety_doc)]

use std::{ffi::c_char, net::SocketAddr};

use autd3capi_driver::*;

use autd3_link_remote::{Remote, RemoteOption as RawOption};

#[repr(C)]
pub struct RemoteOption {
    pub timeout: OptionDuration,
}

impl From<RemoteOption> for RawOption {
    fn from(opt: RemoteOption) -> Self {
        Self {
            timeout: opt.timeout.into(),
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemote(addr: *const c_char, option: RemoteOption) -> ResultLink {
    let addr = if addr.is_null() {
        ""
    } else {
        validate_cstr!(addr, LinkPtr, ResultLink)
    };
    addr.parse::<SocketAddr>()
        .map(|addr| Remote::new(addr, option.into()))
        .into()
}
