#![allow(clippy::missing_safety_doc)]

mod timer_strategy;

use std::{
    ffi::{c_char, CStr, CString},
    net::SocketAddr,
    time::Duration,
};

use autd3capi_driver::*;

use autd3_link_soem::{local::link_soem::*, remote::link_soem_remote::*, EthernetAdapters};
use timer_strategy::TimerStrategy;

#[repr(C)]
pub struct EthernetAdaptersPtr(pub *const libc::c_void);

impl_ptr!(EthernetAdaptersPtr, EthernetAdapters);

#[no_mangle]
pub unsafe extern "C" fn AUTDAUTDLinkSOEMTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDAdapterPointer() -> EthernetAdaptersPtr {
    EthernetAdaptersPtr(Box::into_raw(Box::new(EthernetAdapters::new())) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDAdapterGetSize(adapters: EthernetAdaptersPtr) -> u32 {
    adapters.len() as u32
}

#[no_mangle]
pub unsafe extern "C" fn AUTDAdapterGetAdapter(
    adapters: EthernetAdaptersPtr,
    idx: u32,
    desc: *mut c_char,
    name: *mut c_char,
) {
    let adapter = &adapters[idx as usize];
    let name_ = std::ffi::CString::new(adapter.name().to_string()).unwrap();
    libc::strcpy(name, name_.as_ptr());
    let desc_ = std::ffi::CString::new(adapter.desc().to_string()).unwrap();
    libc::strcpy(desc, desc_.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDAdapterPointerDelete(adapters: EthernetAdaptersPtr) {
    let _ = take!(adapters, EthernetAdapters);
}

#[repr(C)]
pub struct LinkSOEMBuilderPtr(pub *const libc::c_void);

impl LinkSOEMBuilderPtr {
    pub fn new(builder: SOEMBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEM() -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(SOEM::builder())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithSendCycle(
    soem: LinkSOEMBuilderPtr,
    cycle: u64,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(take!(soem, SOEMBuilder).with_send_cycle(cycle))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithSync0Cycle(
    soem: LinkSOEMBuilderPtr,
    cycle: u64,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(take!(soem, SOEMBuilder).with_sync0_cycle(cycle))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithBufSize(
    soem: LinkSOEMBuilderPtr,
    buf_size: u32,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(take!(soem, SOEMBuilder).with_buf_size(buf_size as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithTimerStrategy(
    soem: LinkSOEMBuilderPtr,
    timer_strategy: TimerStrategy,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(take!(soem, SOEMBuilder).with_timer_strategy(timer_strategy.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithSyncMode(
    soem: LinkSOEMBuilderPtr,
    mode: SyncMode,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(take!(soem, SOEMBuilder).with_sync_mode(mode.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithSyncTolerance(
    soem: LinkSOEMBuilderPtr,
    tolerance_ns: u64,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(
        take!(soem, SOEMBuilder).with_sync_tolerance(std::time::Duration::from_nanos(tolerance_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithSyncTimeout(
    soem: LinkSOEMBuilderPtr,
    timeout_ns: u64,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(
        take!(soem, SOEMBuilder).with_sync_timeout(std::time::Duration::from_nanos(timeout_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithIfname(
    soem: LinkSOEMBuilderPtr,
    ifname: *const c_char,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(
        take!(soem, SOEMBuilder).with_ifname(CStr::from_ptr(ifname).to_str().unwrap()),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithStateCheckInterval(
    soem: LinkSOEMBuilderPtr,
    interval_ms: u32,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(
        take!(soem, SOEMBuilder).with_state_check_interval(Duration::from_millis(interval_ms as _)),
    )
}

#[repr(u8)]
pub enum Status {
    Error = 0,
    StateChanged = 1,
    Lost = 2,
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMStatusGetMsg(src: Status, dst: *mut c_char) -> u32 {
    let msg = format!(
        "{}",
        match src {
            Status::Error => autd3_link_soem::Status::Error,
            Status::StateChanged => autd3_link_soem::Status::StateChanged,
            Status::Lost => autd3_link_soem::Status::Lost,
        }
    );
    if dst.is_null() {
        return msg.as_bytes().len() as u32 + 1;
    }
    let c_string = CString::new(msg).unwrap();
    let c_str: &CStr = c_string.as_c_str();
    libc::strcpy(dst, c_str.as_ptr());
    0
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithErrHandler(
    soem: LinkSOEMBuilderPtr,
    handler: ConstPtr,
    context: ConstPtr,
) -> LinkSOEMBuilderPtr {
    if handler.0.is_null() {
        return soem;
    }

    let out_func = move |slave: usize, status: autd3_link_soem::Status| {
        let (out_f, context) = {
            (
                std::mem::transmute::<ConstPtr, unsafe extern "C" fn(ConstPtr, u32, Status)>(
                    handler,
                ),
                context,
            )
        };
        match status {
            autd3_link_soem::Status::Error => {
                out_f(context, slave as _, Status::Error);
            }
            autd3_link_soem::Status::StateChanged => {
                out_f(context, slave as _, Status::StateChanged);
            }
            autd3_link_soem::Status::Lost => {
                out_f(context, slave as _, Status::Lost);
            }
        }
    };
    LinkSOEMBuilderPtr::new(take!(soem, SOEMBuilder).with_err_handler(out_func))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithTimeout(
    soem: LinkSOEMBuilderPtr,
    timeout_ns: u64,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(take!(soem, SOEMBuilder).with_timeout(Duration::from_nanos(timeout_ns)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMIntoBuilder(soem: LinkSOEMBuilderPtr) -> LinkBuilderPtr {
    DynamicLinkBuilder::new(*take!(soem, SOEMBuilder))
}

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
pub unsafe extern "C" fn AUTDLinkRemoteSOEMWithTimeout(
    soem: LinkRemoteSOEMBuilderPtr,
    timeout_ns: u64,
) -> LinkRemoteSOEMBuilderPtr {
    LinkRemoteSOEMBuilderPtr::new(
        take!(soem, RemoteSOEMBuilder).with_timeout(Duration::from_nanos(timeout_ns)),
    )
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
