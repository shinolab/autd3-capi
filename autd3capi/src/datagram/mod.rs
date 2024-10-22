pub mod clear;
pub mod debug;
pub mod force_fan;
pub mod phase_corr;
pub mod pulse_width_encoder;
pub mod reads_fpga_state;
pub mod segment;
pub mod silencer;
pub mod stm;
pub mod synchronize;
pub mod with_parallel_threshold;
pub mod with_timeout;

use std::time::Duration;

use autd3capi_driver::{
    autd3::derive::{Datagram, Geometry},
    driver::{
        error::AUTDInternalError,
        firmware::operation::{Operation, OperationGenerator},
        geometry::Device,
    },
    DatagramPtr, DynamicDatagram, DynamicOperationGenerator,
};

#[derive(Debug)]
pub struct DynamicDatagramTuple {
    pub d1: Box<DynamicDatagram>,
    pub d2: Box<DynamicDatagram>,
}

pub struct DynamicOperationGeneratorTuple {
    pub g1: DynamicOperationGenerator,
    pub g2: DynamicOperationGenerator,
}

impl OperationGenerator for DynamicOperationGeneratorTuple {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&mut self, device: &Device) -> (Self::O1, Self::O2) {
        (self.g1.generate(device).0, self.g2.generate(device).0)
    }
}

impl Datagram for DynamicDatagramTuple {
    fn operation_generator(self, geometry: &Geometry) -> Result<Self::G, AUTDInternalError> {
        Ok(DynamicOperationGeneratorTuple {
            g1: self.d1.operation_generator(geometry)?,
            g2: self.d2.operation_generator(geometry)?,
        })
    }

    fn timeout(&self) -> Option<Duration> {
        match (self.d1.timeout(), self.d2.timeout()) {
            (Some(t1), Some(t2)) => Some(t1.max(t2)),
            (a, b) => a.or(b),
        }
    }

    fn parallel_threshold(&self) -> Option<usize> {
        match (self.d1.parallel_threshold(), self.d2.parallel_threshold()) {
            (Some(t1), Some(t2)) => Some(t1.min(t2)),
            (a, b) => a.or(b),
        }
    }

    type G = DynamicOperationGeneratorTuple;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramTuple(d1: DatagramPtr, d2: DatagramPtr) -> DatagramPtr {
    DynamicDatagramTuple {
        d1: d1.into(),
        d2: d2.into(),
    }
    .into()
}
