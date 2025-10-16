#![allow(clippy::missing_safety_doc)]

mod autd3_device;
mod datagram;
mod duration;
mod firmware;
mod ptr;
mod result;

pub use autd3_device::*;
pub use datagram::*;
pub use duration::*;
pub use firmware::*;
pub use ptr::*;
pub use result::*;

pub use autd3;
pub use autd3::core;
pub use autd3::driver;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConstPtr(pub *const std::ffi::c_void);

unsafe impl Send for ConstPtr {}
unsafe impl Sync for ConstPtr {}

pub use autd3::driver::geometry::Point3;

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

pub unsafe fn strcpy(dst: *mut std::ffi::c_char, src: *const std::ffi::c_char) {
    unsafe {
        let mut dst = dst;
        let mut src = src;
        while *src != 0 {
            *dst = *src;
            dst = dst.add(1);
            src = src.add(1);
        }
    }
}
