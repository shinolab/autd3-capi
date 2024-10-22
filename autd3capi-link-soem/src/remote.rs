use std::{
    ffi::{c_char, CStr},
    net::SocketAddr,
};

use autd3capi_driver::*;

use autd3_link_soem::remote::link_soem_remote::*;

#[repr(C)]
pub struct LinkRemoteSOEMBuilderPtr(pub *const libc::c_void);

impl LinkRemoteSOEMBuilderPtr {
    pub fn new(builder: RemoteSOEMBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[repr(C)]
pub struct ResultLinkRemoteSOEMBuilder {
    pub result: LinkRemoteSOEMBuilderPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteSOEM(addr: *const c_char) -> ResultLinkRemoteSOEMBuilder {
    let addr = match CStr::from_ptr(addr).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultLinkRemoteSOEMBuilder {
                result: LinkRemoteSOEMBuilderPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
            };
        }
    };
    let addr = match addr.parse::<SocketAddr>() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultLinkRemoteSOEMBuilder {
                result: LinkRemoteSOEMBuilderPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
            };
        }
    };
    ResultLinkRemoteSOEMBuilder {
        result: LinkRemoteSOEMBuilderPtr::new(RemoteSOEM::builder(addr)),
        err_len: 0,
        err: ConstPtr(std::ptr::null_mut()),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteSOEMIntoBuilder(
    soem: LinkRemoteSOEMBuilderPtr,
) -> LinkBuilderPtr {
    #[cfg(feature = "static")]
    {
        DynamicLinkBuilder::new(*take!(soem, RemoteSOEMBuilder))
    }
    #[cfg(not(feature = "static"))]
    {
        DynamicLinkBuilder::new(SyncLinkBuilder {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            inner: *take!(soem, RemoteSOEMBuilder),
        })
    }
}
