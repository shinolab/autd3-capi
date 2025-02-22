use autd3::{
    Controller,
    controller::{Sender, Sleep},
};
use autd3_core::link::Link;

use crate::impl_ptr;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ControllerPtr(pub *const libc::c_void);

impl_ptr!(ControllerPtr, Controller<Box<dyn Link>>);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SenderPtr(pub *const libc::c_void);

impl_ptr!(SenderPtr, Sender<'static, Box<dyn Link>, Box<dyn Sleep>>);
