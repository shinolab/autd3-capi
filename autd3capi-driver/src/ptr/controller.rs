use autd3::{
    Controller,
    core::{link::Link, sleep::Sleep},
    driver::firmware::{
        auto::{Auto, transmission::Sender},
        driver::TimerStrategy,
    },
};

use crate::impl_ptr;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ControllerPtr(pub *const libc::c_void);

impl_ptr!(ControllerPtr, Controller<Box<dyn Link>, Auto>);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SenderPtr(pub *const libc::c_void);

impl_ptr!(
    SenderPtr,
    Sender<'static, Box<dyn Link>, Box<dyn Sleep>, Box<dyn TimerStrategy<Box<dyn Sleep>>>>
);
