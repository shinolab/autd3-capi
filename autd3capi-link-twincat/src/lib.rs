#![allow(clippy::missing_safety_doc)]

use std::ffi::{CStr, c_char};

use autd3capi_driver::*;

use autd3_link_twincat::{local::*, remote::*};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCAT() -> ResultLink {
    TwinCAT::new().into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCAT(
    server_ams_net_id: *const c_char,
    server_ip: *const c_char,
    client_ams_net_id: *const c_char,
) -> ResultLink {
    let server_ip = if server_ip.is_null() {
        ""
    } else {
        validate_cstr!(server_ip, LinkPtr, ResultLink)
    }
    .to_owned();
    let client_ams_net_id = if client_ams_net_id.is_null() {
        ""
    } else {
        validate_cstr!(client_ams_net_id, LinkPtr, ResultLink)
    }
    .to_owned();
    unsafe {
        CStr::from_ptr(server_ams_net_id)
            .to_str()
            .map(|path| {
                RemoteTwinCAT::new(
                    path,
                    RemoteTwinCATOption {
                        server_ip,
                        client_ams_net_id,
                    },
                )
            })
            .into()
    }
}
