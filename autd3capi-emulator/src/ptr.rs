use autd3_emulator::{Emulator, Instant, Record, Recorder, Rms};
use autd3capi_driver::{autd3::Controller, impl_ptr, libc};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EmulatorControllerPtr(pub *const libc::c_void);

impl_ptr!(EmulatorControllerPtr, Controller<Recorder>);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EmulatorPtr(pub *const libc::c_void);

impl_ptr!(EmulatorPtr, Emulator);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RecordPtr(pub *const libc::c_void);

impl_ptr!(RecordPtr, Record);

impl RecordPtr {
    pub fn static_deref(&self) -> &'static Record {
        unsafe { (self.0 as *const Record).as_ref().unwrap() }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InstantPtr(pub *const libc::c_void);

impl_ptr!(InstantPtr, Instant<'static>);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RmsPtr(pub *const libc::c_void);

impl_ptr!(RmsPtr, Rms);
