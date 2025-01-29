#![allow(clippy::missing_safety_doc)]

use std::{ffi::c_char, net::SocketAddr};

use autd3capi_driver::*;

use autd3_link_simulator::*;

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorTracingInitWithFile(path: *const c_char) -> ResultStatus {
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
            AUTDStatus::AUTDTrue
        })
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulator(addr: *const c_char) -> ResultLink {
    let addr = if addr.is_null() {
        ""
    } else {
        validate_cstr!(addr, LinkPtr, ResultLink)
    };
    addr.parse::<SocketAddr>().map(Simulator::new).into()
}
