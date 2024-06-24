#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr},
    time::Duration,
};

use autd3capi_driver::*;

use autd3_link_twincat::{local::twincat_link::*, remote::remote_twincat_link::*};

#[repr(C)]
pub struct LinkTwinCATBuilderPtr(pub *const libc::c_void);

impl LinkTwinCATBuilderPtr {
    pub fn new(builder: TwinCATBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCAT() -> LinkTwinCATBuilderPtr {
    LinkTwinCATBuilderPtr::new(TwinCAT::builder())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCATWithTimeout(
    twincat: LinkTwinCATBuilderPtr,
    timeout_ns: u64,
) -> LinkTwinCATBuilderPtr {
    LinkTwinCATBuilderPtr::new(
        take!(twincat, TwinCATBuilder).with_timeout(Duration::from_nanos(timeout_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCATIntoBuilder(
    twincat: LinkTwinCATBuilderPtr,
) -> LinkBuilderPtr {
    DynamicLinkBuilder::new(*take!(twincat, TwinCATBuilder))
}

#[repr(C)]

pub struct LinkRemoteTwinCATBuilderPtr(pub *const libc::c_void);

impl LinkRemoteTwinCATBuilderPtr {
    pub fn new(builder: RemoteTwinCATBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[repr(C)]

pub struct ResultLinkRemoteTwinCATBuilder {
    pub result: LinkRemoteTwinCATBuilderPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCAT(
    server_ams_net_id: *const c_char,
) -> ResultLinkRemoteTwinCATBuilder {
    match CStr::from_ptr(server_ams_net_id).to_str() {
        Ok(v) => {
            let builder = RemoteTwinCAT::builder(v);
            ResultLinkRemoteTwinCATBuilder {
                result: LinkRemoteTwinCATBuilderPtr::new(builder),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            }
        }
        Err(e) => {
            let err = e.to_string();
            ResultLinkRemoteTwinCATBuilder {
                result: LinkRemoteTwinCATBuilderPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATWithServerIP(
    twincat: LinkRemoteTwinCATBuilderPtr,
    addr: *const c_char,
) -> LinkRemoteTwinCATBuilderPtr {
    LinkRemoteTwinCATBuilderPtr::new(
        take!(twincat, RemoteTwinCATBuilder).with_server_ip(CStr::from_ptr(addr).to_str().unwrap()),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATWithClientAmsNetId(
    twincat: LinkRemoteTwinCATBuilderPtr,
    id: *const c_char,
) -> LinkRemoteTwinCATBuilderPtr {
    LinkRemoteTwinCATBuilderPtr::new(
        take!(twincat, RemoteTwinCATBuilder)
            .with_client_ams_net_id(CStr::from_ptr(id).to_str().unwrap()),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATWithTimeout(
    twincat: LinkRemoteTwinCATBuilderPtr,
    timeout_ns: u64,
) -> LinkRemoteTwinCATBuilderPtr {
    LinkRemoteTwinCATBuilderPtr::new(
        take!(twincat, RemoteTwinCATBuilder).with_timeout(Duration::from_nanos(timeout_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATIntoBuilder(
    twincat: LinkRemoteTwinCATBuilderPtr,
) -> LinkBuilderPtr {
    DynamicLinkBuilder::new(*take!(twincat, RemoteTwinCATBuilder))
}
