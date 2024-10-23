use std::time::Duration;

use autd3::derive::{Datagram, Geometry};
use autd3_driver::{
    error::AUTDInternalError,
    firmware::operation::{Operation, OperationGenerator},
    geometry::Device,
};

use super::{DynamicDatagram, DynamicOperationGenerator};

pub struct DynamicDatagramTuple {
    pub d1: Box<DynamicDatagram>,
    pub d2: Box<DynamicDatagram>,
}

impl std::fmt::Debug for DynamicDatagramTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.d1, self.d2)
    }
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
