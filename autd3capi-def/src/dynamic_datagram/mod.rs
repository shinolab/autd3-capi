#![allow(clippy::missing_safety_doc)]

mod clear;
mod debug;
mod force_fan;
mod gain;
mod modulation;
mod phase_filter;
mod reads_fpga_state;
mod silencer;
mod stm;
mod synchronize;
mod with_segment;

pub use debug::*;
pub use force_fan::*;
pub use phase_filter::*;
pub use reads_fpga_state::*;
pub use with_segment::*;

use std::time::Duration;

use autd3_driver::{datagram::Datagram, error::AUTDInternalError, operation::Operation};

pub trait DynamicDatagram {
    #[allow(clippy::type_complexity)]
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError>;

    fn timeout(&self) -> Option<Duration>;
}

pub struct DynamicDatagramPack {
    pub d: Box<Box<dyn DynamicDatagram>>,
    pub timeout: Option<std::time::Duration>,
}

impl Datagram for DynamicDatagramPack {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let Self { mut d, .. } = self;
        d.operation()
    }

    fn timeout(&self) -> Option<Duration> {
        if self.timeout.is_some() {
            self.timeout
        } else {
            self.d.timeout()
        }
    }
}

unsafe impl Send for DynamicDatagramPack {}
unsafe impl Sync for DynamicDatagramPack {}

pub struct DynamicDatagramPack2 {
    pub d1: Box<Box<dyn DynamicDatagram>>,
    pub d2: Box<Box<dyn DynamicDatagram>>,
    pub timeout: Option<std::time::Duration>,
}

impl Datagram for DynamicDatagramPack2 {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let Self { mut d1, mut d2, .. } = self;
        let (op1, _) = d1.operation()?;
        let (op2, _) = d2.operation()?;
        Ok((op1, op2))
    }

    fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

unsafe impl Send for DynamicDatagramPack2 {}
unsafe impl Sync for DynamicDatagramPack2 {}
