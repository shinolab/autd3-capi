use autd3::Controller;
use autd3_driver::link::Link;

use crate::impl_ptr;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ControllerPtr(pub *const libc::c_void);

unsafe impl Send for ControllerPtr {}
unsafe impl Sync for ControllerPtr {}

impl_ptr!(ControllerPtr, Controller<Box<dyn Link>>);
