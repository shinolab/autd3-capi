#![allow(clippy::missing_safety_doc)]

pub mod adapter;
pub mod local;
pub mod process_priority;
pub mod remote;
pub mod status;
pub mod thread_priority;
pub mod timer_strategy;

use std::{
    ffi::{c_char, CStr, CString},
    num::{NonZeroU64, NonZeroUsize},
    time::Duration,
};

use autd3capi_driver::*;

use autd3_link_soem::{local::link_soem::*, ThreadPriority};
use process_priority::ProcessPriority;
use status::Status;
use thread_priority::ThreadPriorityPtr;
use timer_strategy::TimerStrategy;

#[no_mangle]
pub unsafe extern "C" fn AUTDAUTDLinkSOEMTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
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
    LinkSOEMBuilderPtr::new(
        take!(soem, SOEMBuilder).with_send_cycle(NonZeroU64::new_unchecked(cycle)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithSync0Cycle(
    soem: LinkSOEMBuilderPtr,
    cycle: u64,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(
        take!(soem, SOEMBuilder).with_sync0_cycle(NonZeroU64::new_unchecked(cycle)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithBufSize(
    soem: LinkSOEMBuilderPtr,
    buf_size: u32,
) -> LinkSOEMBuilderPtr {
    LinkSOEMBuilderPtr::new(
        take!(soem, SOEMBuilder).with_buf_size(NonZeroUsize::new_unchecked(buf_size as _)),
    )
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

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMStatusGetMsg(src: Status, dst: *mut c_char) -> u32 {
    let msg = format!("{}", autd3_link_soem::Status::from(src));
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
        out_f(context, slave as _, status.into());
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
pub unsafe extern "C" fn AUTDLinkSOEMWithProcessPriority(
    soem: LinkSOEMBuilderPtr,
    priority: ProcessPriority,
) -> LinkSOEMBuilderPtr {
    #[cfg(target_os = "windows")]
    {
        LinkSOEMBuilderPtr::new(take!(soem, SOEMBuilder).with_process_priority(priority.into()))
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = priority;
        soem
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMWithThreadPriority(
    soem: LinkSOEMBuilderPtr,
    priority: ThreadPriorityPtr,
) -> LinkSOEMBuilderPtr {
    let priority = *take!(priority, ThreadPriority);
    LinkSOEMBuilderPtr::new(take!(soem, SOEMBuilder).with_thread_priority(priority))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMIntoBuilder(soem: LinkSOEMBuilderPtr) -> LinkBuilderPtr {
    #[cfg(feature = "static")]
    {
        DynamicLinkBuilder::new(*take!(soem, SOEMBuilder))
    }
    #[cfg(not(feature = "static"))]
    {
        DynamicLinkBuilder::new(SyncLinkBuilder {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            inner: *take!(soem, SOEMBuilder),
        })
    }
}
