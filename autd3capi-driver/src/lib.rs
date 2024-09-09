#![allow(clippy::missing_safety_doc)]

mod autd3_device;
mod datagram;
mod ethercat;
mod firmware;
mod link;
mod ptr;
mod resampler;
mod result;

pub use autd3_device::*;
pub use datagram::*;
pub use ethercat::*;
pub use firmware::*;
pub use link::*;
pub use ptr::*;
pub use resampler::*;
pub use result::*;

pub use async_ffi;
pub use autd3;
pub use autd3_driver as driver;
pub use libc;
pub use tokio;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConstPtr(pub *const libc::c_void);

unsafe impl Send for ConstPtr {}
unsafe impl Sync for ConstPtr {}

pub type L = dyn autd3_driver::link::Link;
pub type G = dyn autd3_driver::datagram::Gain + Send + Sync;
pub type M = dyn autd3_driver::datagram::Modulation + Send + Sync;

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
