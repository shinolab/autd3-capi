use autd3_emulator::{Emulator, Record, Recorder, SoundField};
use autd3capi_driver::{autd3::Controller, impl_ptr, libc};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EmulatorControllerPtr(pub *const libc::c_void);

unsafe impl Send for EmulatorControllerPtr {}
unsafe impl Sync for EmulatorControllerPtr {}

impl_ptr!(EmulatorControllerPtr, Controller<Recorder>);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EmulatorPtr(pub *const libc::c_void);

unsafe impl Send for EmulatorPtr {}
unsafe impl Sync for EmulatorPtr {}

impl EmulatorPtr {
    pub fn new(emulator: Emulator) -> Self {
        Self(Box::into_raw(Box::new(emulator)) as _)
    }
}

impl_ptr!(EmulatorPtr, Emulator);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RecordPtr(pub *const libc::c_void);

unsafe impl Send for RecordPtr {}
unsafe impl Sync for RecordPtr {}

impl_ptr!(RecordPtr, Record);

impl RecordPtr {
    pub fn static_deref(&self) -> &'static Record {
        unsafe { (self.0 as *const Record).as_ref().unwrap() }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SoundFieldPtr(pub *const libc::c_void);

unsafe impl Send for SoundFieldPtr {}
unsafe impl Sync for SoundFieldPtr {}

impl_ptr!(SoundFieldPtr, SoundField<'static>);
