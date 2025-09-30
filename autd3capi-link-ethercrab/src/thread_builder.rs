use autd3capi_driver::*;

use autd3_link_ethercrab::thread_priority::{ThreadPriority, ThreadPriorityValue};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ThreadPriorityPtr(pub *const libc::c_void);

impl From<ThreadPriority> for ThreadPriorityPtr {
    fn from(v: ThreadPriority) -> Self {
        Self(Box::into_raw(Box::new(v)) as _)
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkEtherCrabThreadPriorityMin() -> ThreadPriorityPtr {
    ThreadPriorityPtr::from(ThreadPriority::Min)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkEtherCrabThreadPriorityCrossplatform(
    value: u8,
) -> ThreadPriorityPtr {
    ThreadPriorityValue::try_from(value)
        .map(ThreadPriority::Crossplatform)
        .map(ThreadPriorityPtr::from)
        .unwrap_or(ThreadPriorityPtr(std::ptr::null()))
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkEtherCrabThreadPriorityMax() -> ThreadPriorityPtr {
    ThreadPriorityPtr::from(ThreadPriority::Max)
}
