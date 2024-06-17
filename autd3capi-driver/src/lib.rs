#![allow(clippy::missing_safety_doc)]

mod autd3_device;
mod datagram;
mod ethercat;
mod firmware;
mod link;
mod ptr;
mod result;

use core::panic;

use autd3::derive::tracing;
pub use autd3_device::*;
pub use datagram::*;
pub use ethercat::*;
pub use firmware::*;
pub use link::*;
pub use ptr::*;
pub use result::*;

pub use async_ffi;
pub use autd3;
pub use autd3_driver as driver;
pub use libc;
pub use tokio;

pub type ConstPtr = *const libc::c_void;
pub type L = dyn autd3_driver::link::Link;
pub type G = dyn autd3_driver::datagram::Gain;
pub type M = dyn autd3_driver::datagram::Modulation;

pub use autd3_driver::geometry::Vector3;

#[macro_export]
macro_rules! vec_from_raw {
    ($src:expr, $type:ty, $len:expr) => {{
        let mut tmp = Vec::<$type>::with_capacity($len as _);
        tmp.set_len($len as _);
        std::ptr::copy_nonoverlapping($src as *const _, tmp.as_mut_ptr(), $len as _);
        tmp
    };};
}

#[macro_export]
macro_rules! take {
    ($ptr:expr, $type:ty) => {
        Box::from_raw($ptr.0 as *mut $type)
    };
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContextPtr(pub ConstPtr);

unsafe impl Send for ContextPtr {}
unsafe impl Sync for ContextPtr {}

pub const TRACE_LEVEL_ERROR: u8 = 1;
pub const TRACE_LEVEL_WARN: u8 = 2;
pub const TRACE_LEVEL_INFO: u8 = 3;
pub const TRACE_LEVEL_DEBUG: u8 = 4;
pub const TRACE_LEVEL_TRACE: u8 = 5;

pub fn trace_level_into(level: u8) -> tracing::Level {
    match level {
        TRACE_LEVEL_ERROR => tracing::Level::ERROR,
        TRACE_LEVEL_WARN => tracing::Level::WARN,
        TRACE_LEVEL_INFO => tracing::Level::INFO,
        TRACE_LEVEL_DEBUG => tracing::Level::DEBUG,
        TRACE_LEVEL_TRACE => tracing::Level::TRACE,
        _ => panic!("Invalid trace level: {}", level),
    }
}
