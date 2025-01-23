use autd3_core::{
    datagram::{Datagram, DatagramOption},
    geometry::Geometry,
};
use autd3_driver::{
    error::AUTDDriverError,
    firmware::operation::{BoxedOperation, OperationGenerator},
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
    type O1 = BoxedOperation;
    type O2 = BoxedOperation;

    fn generate(&mut self, device: &Device) -> (Self::O1, Self::O2) {
        (self.g1.generate(device).0, self.g2.generate(device).0)
    }
}

impl Datagram for DynDatagramTuple {
    type G = DOperationGeneratorTuple;
    type Error = AUTDDriverError;

    fn operation_generator(
        self,
        geometry: &Geometry,
        option: &DatagramOption,
    ) -> Result<Self::G, Self::Error> {
        Ok(DOperationGeneratorTuple {
            g1: self.d1.operation_generator(geometry, option)?,
            g2: self.d2.operation_generator(geometry, option)?,
        })
    }

    fn option(&self) -> autd3_core::datagram::DatagramOption {
        DatagramOption {
            timeout: self.d1.option().timeout.max(self.d2.option().timeout),
            parallel_threshold: self
                .d1
                .option()
                .parallel_threshold
                .min(self.d2.option().parallel_threshold),
        }
    }
}
