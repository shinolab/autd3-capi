#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr},
    net::Ipv4Addr,
    time::Duration,
};

use async_ffi::{FfiFuture, FutureExt};
use autd3capi_driver::*;

use autd3_link_simulator::*;

#[repr(C)]

pub struct LinkSimulatorBuilderPtr(pub *const libc::c_void);

impl LinkSimulatorBuilderPtr {
    pub fn new(builder: SimulatorBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[repr(C)]

pub struct ResultLinkSimulatorBuilder {
    pub result: LinkSimulatorBuilderPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulator(port: u16) -> LinkSimulatorBuilderPtr {
    LinkSimulatorBuilderPtr::new(Simulator::builder(port))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulatorWithAddr(
    simulator: LinkSimulatorBuilderPtr,
    addr: *const c_char,
) -> ResultLinkSimulatorBuilder {
    let addr = match CStr::from_ptr(addr).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultLinkSimulatorBuilder {
                result: LinkSimulatorBuilderPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
            };
        }
    };
    let addr = match addr.parse::<Ipv4Addr>() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultLinkSimulatorBuilder {
                result: LinkSimulatorBuilderPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
            };
        }
    };
    ResultLinkSimulatorBuilder {
        result: LinkSimulatorBuilderPtr::new(
            take!(simulator, SimulatorBuilder).with_server_ip(addr),
        ),
        err_len: 0,
        err: ConstPtr(std::ptr::null_mut()),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulatorWithTimeout(
    simulator: LinkSimulatorBuilderPtr,
    timeout_ns: u64,
) -> LinkSimulatorBuilderPtr {
    LinkSimulatorBuilderPtr::new(
        take!(simulator, SimulatorBuilder).with_timeout(Duration::from_nanos(timeout_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulatorIntoBuilder(
    simulator: LinkSimulatorBuilderPtr,
) -> LinkBuilderPtr {
    #[cfg(feature = "static")]
    {
        DynamicLinkBuilder::new(*take!(simulator, SimulatorBuilder))
    }
    #[cfg(not(feature = "static"))]
    {
        DynamicLinkBuilder::new(SyncLinkBuilder {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            inner: *take!(simulator, SimulatorBuilder),
        })
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulatorUpdateGeometry(
    mut simulator: LinkPtr,
    geometry: GeometryPtr,
) -> FfiFuture<ResultI32> {
    #[cfg(feature = "static")]
    {
        async move {
            let r: ResultI32 = simulator
                .cast_mut::<Simulator>()
                .update_geometry(&geometry)
                .await
                .into();
            r
        }
        .into_ffi()
    }
    #[cfg(not(feature = "static"))]
    {
        async move {
            let link = simulator.cast_mut::<SyncLink<Simulator>>();
            let r: ResultI32 = link
                .runtime
                .block_on(link.inner.update_geometry(&geometry))
                .into();
            r
        }
        .into_ffi()
    }
}
