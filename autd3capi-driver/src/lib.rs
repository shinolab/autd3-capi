#![allow(clippy::missing_safety_doc)]

mod autd3_device;
mod firmware;
mod link;
mod ptr;
mod result;

pub use autd3_device::*;
pub use firmware::*;
pub use link::*;
pub use ptr::*;
pub use result::*;

pub type ConstPtr = *const libc::c_void;
pub type L = dyn autd3_driver::link::Link;
pub type G = dyn autd3_driver::datagram::Gain;
pub type M = dyn autd3_driver::datagram::Modulation;

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
