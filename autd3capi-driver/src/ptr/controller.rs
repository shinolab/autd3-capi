use autd3::{controller::ControllerBuilder, Controller};
use autd3_driver::link::Link;

use crate::impl_ptr;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ControllerPtr(pub *const libc::c_void);

impl_ptr!(ControllerPtr, Controller<Box<dyn Link>>);

#[repr(C)]
pub struct ControllerBuilderPtr(pub *const libc::c_void);

impl_ptr!(ControllerBuilderPtr, ControllerBuilder);
