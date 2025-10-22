use autd3::{
    Controller, core::link::Link, driver::firmware::transmission::Sender, prelude::StdSleeper,
};

use crate::impl_ptr;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ControllerPtr(pub *const std::ffi::c_void);

impl_ptr!(ControllerPtr, Controller<Box<dyn Link>>);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SenderPtr(pub *const std::ffi::c_void);

impl_ptr!(SenderPtr, Sender<'static, Box<dyn Link>, StdSleeper>);
