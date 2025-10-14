use autd3::{
    Controller,
    core::{link::Link, sleep::Sleeper},
    driver::firmware::transmission::Sender,
};

use crate::impl_ptr;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ControllerPtr(pub *const std::ffi::c_void);

impl_ptr!(ControllerPtr, Controller<Box<dyn Link>>);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SenderPtr(pub *const std::ffi::c_void);

impl_ptr!(SenderPtr, Sender<'static, Box<dyn Link>, Box<dyn Sleeper>>);
