#![allow(clippy::missing_safety_doc)]

mod controller;
mod custom;
mod drive;
mod dynamic_datagram;
mod link;
mod result;
mod sampling_config;

use std::ops::Deref;

pub use autd3::{controller::Controller, error::AUTDError};
pub use autd3_driver::{
    datagram::{Datagram, Gain, GainFilter, Modulation, STMProps},
    defined::float,
    error::AUTDInternalError,
    firmware_version::FirmwareInfo,
    geometry::{Device, Geometry, Vector3},
    link::{Link, LinkBuilder},
};
pub use controller::*;
pub use custom::{CustomGain, CustomModulation};
pub use drive::*;
pub use dynamic_datagram::{
    DynamicConfigureDebugOutputIdx, DynamicConfigureForceFan, DynamicConfigureModDelay,
    DynamicConfigureReadsFPGAState, DynamicDatagram, DynamicDatagramPack, DynamicDatagramPack2,
};
pub use link::*;

pub use libc::c_void;
pub use result::*;
pub use sampling_config::*;

pub use autd3;
pub use autd3_driver as driver;
pub use libc;
pub use tokio;

pub type ConstPtr = *const c_void;
pub type L = dyn Link;
pub type G = dyn Gain;
pub type M = dyn Modulation;
pub type Cnt = SyncController;

pub const NUM_TRANS_IN_UNIT: u32 = 249;
pub const NUM_TRANS_IN_X: u32 = 18;
pub const NUM_TRANS_IN_Y: u32 = 14;
pub const TRANS_SPACING_MM: float = 10.16;
pub const DEVICE_HEIGHT_MM: float = 151.4;
pub const DEVICE_WIDTH_MM: float = 192.0;
pub const FPGA_CLK_FREQ: u32 = 20480000;
pub const ULTRASOUND_FREQUENCY: float = 40000.0;

pub const AUTD3_ERR: i32 = -1;
pub const AUTD3_TRUE: i32 = 1;
pub const AUTD3_FALSE: i32 = 0;

#[macro_export]
macro_rules! cast {
    ($ptr:expr, $type:ty) => {
        ($ptr as *const $type).as_ref().unwrap()
    };
}

#[macro_export]
macro_rules! cast_mut {
    ($ptr:expr, $type:ty) => {
        ($ptr as *mut $type).as_mut().unwrap()
    };
}

#[repr(u8)]
pub enum GainSTMMode {
    PhaseIntensityFull = 0,
    PhaseFull = 1,
    PhaseHalf = 2,
}

impl From<GainSTMMode> for autd3::prelude::GainSTMMode {
    fn from(mode: GainSTMMode) -> Self {
        match mode {
            GainSTMMode::PhaseIntensityFull => autd3::prelude::GainSTMMode::PhaseIntensityFull,
            GainSTMMode::PhaseFull => autd3::prelude::GainSTMMode::PhaseFull,
            GainSTMMode::PhaseHalf => autd3::prelude::GainSTMMode::PhaseHalf,
        }
    }
}

#[repr(u8)]
pub enum TimerStrategy {
    Sleep = 0,
    BusyWait = 1,
    NativeTimer = 2,
}

impl From<TimerStrategy> for autd3::prelude::TimerStrategy {
    fn from(strategy: TimerStrategy) -> Self {
        match strategy {
            TimerStrategy::Sleep => autd3::prelude::TimerStrategy::Sleep,
            TimerStrategy::NativeTimer => autd3::prelude::TimerStrategy::NativeTimer,
            TimerStrategy::BusyWait => autd3::prelude::TimerStrategy::BusyWait,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FirmwareInfoListPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GainCalcDrivesMapPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ModulationCalcPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeometryPtr(pub ConstPtr);

unsafe impl Send for GeometryPtr {}
unsafe impl Sync for GeometryPtr {}

impl Deref for GeometryPtr {
    type Target = Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *const Geometry).as_ref().unwrap() }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DevicePtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TransducerPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DatagramPtr(pub ConstPtr);

impl DatagramPtr {
    pub fn new<T: DynamicDatagram>(d: T) -> Self {
        let d: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(d));
        Self(Box::into_raw(d) as _)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl From<DatagramPtr> for Box<Box<dyn DynamicDatagram>> {
    fn from(value: DatagramPtr) -> Self {
        unsafe { Box::from_raw(value.0 as *mut Box<dyn DynamicDatagram>) }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GainPtr(pub ConstPtr);

impl GainPtr {
    pub fn new<T: Gain + 'static>(g: T) -> Self {
        let g: Box<Box<G>> = Box::new(Box::new(g));
        Self(Box::into_raw(g) as _)
    }
}

#[macro_export]
macro_rules! take_gain {
    ($ptr:expr, $type:ty) => {
        Box::from_raw($ptr.0 as *mut Box<G> as *mut Box<$type>)
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ModulationPtr(pub ConstPtr);

impl ModulationPtr {
    pub fn new<T: Modulation + 'static>(m: T) -> Self {
        let m: Box<Box<M>> = Box::new(Box::new(m));
        Self(Box::into_raw(m) as _)
    }
}

#[macro_export]
macro_rules! take_mod {
    ($ptr:expr, $type:ty) => {
        Box::from_raw($ptr.0 as *mut Box<M> as *mut Box<$type>)
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CachePtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct STMPropsPtr(pub ConstPtr);

impl STMPropsPtr {
    pub fn new(props: STMProps) -> Self {
        Self(Box::into_raw(Box::new(props)) as _)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GroupGainMapPtr(pub ConstPtr);
