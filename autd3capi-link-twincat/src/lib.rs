#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};

use autd3capi_driver::*;

use autd3_link_twincat::{local::*, remote::remote_twincat_link::*};

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTwinCATTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTwinCATTracingInitWithFile(
    path: *const c_char,
) -> ResultStatus {
    let path = validate_cstr!(path, AUTDStatus, ResultStatus);
    std::fs::File::options()
        .append(true)
        .create(true)
  
        .open(path)
        .map(|f| {
            tracing_subscriber::fmt()
                .with_writer(f)
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_ansi(false)
                .init();
            AUTDStatus::TRUE
        })
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCAT() -> LinkBuilderPtr {
    TwinCAT::builder().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCAT(
    server_ams_net_id: *const c_char,
    server_ip: *const c_char,
    client_ams_net_id: *const c_char,
) -> ResultLinkBuilder {
    let server_ip = if server_ip.is_null() {
        ""
    } else {
        validate_cstr!(server_ip, LinkBuilderPtr, ResultLinkBuilder)
    };
    let client_ams_net_id = if client_ams_net_id.is_null() {
        ""
    } else {
        validate_cstr!(client_ams_net_id, LinkBuilderPtr, ResultLinkBuilder)
    };
    CStr::from_ptr(server_ams_net_id)
        .to_str()
        .map(|path| {
            RemoteTwinCAT::builder(path)
                .with_server_ip(server_ip)
                .with_client_ams_net_id(client_ams_net_id)
        })
        .into()
}
