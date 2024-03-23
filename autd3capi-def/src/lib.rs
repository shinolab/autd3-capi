#![allow(clippy::missing_safety_doc)]

mod common;
mod controller;
mod dynamic_datagram;
mod gain;
mod link;
mod modulation;
mod ptr;

pub use autd3::{controller::Controller, error::AUTDError};
pub use autd3_driver::{
    datagram::{Datagram, Gain, GainFilter, Modulation, STMProps},
    error::AUTDInternalError,
    firmware_version::FirmwareInfo,
    geometry::{Device, Geometry, Vector3},
    link::{Link, LinkBuilder},
};

pub use common::*;
pub use controller::*;
pub use dynamic_datagram::*;
pub use gain::*;
pub use link::*;
pub use modulation::*;
pub use ptr::*;

pub use autd3;
pub use autd3_driver as driver;
pub use libc;
pub use tokio;

pub type ConstPtr = *const libc::c_void;
pub type L = dyn Link;
pub type G = dyn Gain;
pub type M = dyn Modulation;
pub type Cnt = SyncController;

pub const NUM_TRANS_IN_UNIT: u32 = 249;
pub const NUM_TRANS_IN_X: u32 = 18;
pub const NUM_TRANS_IN_Y: u32 = 14;
pub const TRANS_SPACING_MM: f64 = 10.16;
pub const DEVICE_HEIGHT_MM: f64 = 151.4;
pub const DEVICE_WIDTH_MM: f64 = 192.0;
pub const FPGA_CLK_FREQ: u32 = 20480000;
pub const ULTRASOUND_FREQUENCY: f64 = 40000.0;

pub const AUTD3_ERR: i32 = -1;
pub const AUTD3_TRUE: i32 = 1;
pub const AUTD3_FALSE: i32 = 0;

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
