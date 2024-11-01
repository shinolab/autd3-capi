use std::time::Duration;

use autd3::derive::{Datagram, Geometry};
use autd3_driver::{
    error::AUTDInternalError,
    firmware::operation::{Operation, OperationGenerator},
    geometry::Device,
};

use super::{DynDatagram, DynOperationGenerator};

pub struct DynDatagramTuple {
    pub d1: Box<DynDatagram>,
    pub d2: Box<DynDatagram>,
}

impl std::fmt::Debug for DynDatagramTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.d1, self.d2)
    }
}

pub struct DOperationGeneratorTuple {
    pub g1: DynOperationGenerator,
    pub g2: DynOperationGenerator,
}

impl OperationGenerator for DOperationGeneratorTuple {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&mut self, device: &Device) -> (Self::O1, Self::O2) {
        (self.g1.generate(device).0, self.g2.generate(device).0)
    }
}

impl Datagram for DynDatagramTuple {
    fn operation_generator(self, geometry: &Geometry) -> Result<Self::G, AUTDInternalError> {
        Ok(DOperationGeneratorTuple {
            g1: self.d1.operation_generator(geometry)?,
            g2: self.d2.operation_generator(geometry)?,
        })
    }

    fn timeout(&self) -> Option<Duration> {
        self.d1
            .timeout()
            .into_iter()
            .chain(self.d2.timeout().into_iter())
            .max()
    }

    fn parallel_threshold(&self) -> Option<usize> {
        self.d1
            .parallel_threshold()
            .into_iter()
            .chain(self.d2.parallel_threshold().into_iter())
            .min()
    }

    type G = DOperationGeneratorTuple;
}
