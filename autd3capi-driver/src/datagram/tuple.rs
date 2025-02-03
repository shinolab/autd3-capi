use autd3_core::{
    datagram::{Datagram, DatagramOption},
    geometry::Geometry,
};
use autd3_driver::{
    datagram::BoxedDatagram,
    error::AUTDDriverError,
    firmware::operation::{BoxedOperation, OperationGenerator},
    geometry::Device,
};

pub struct DynDatagramTuple {
    pub d1: BoxedDatagram,
    pub d2: BoxedDatagram,
}

impl std::fmt::Debug for DynDatagramTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.d1, self.d2)
    }
}

pub struct DOperationGeneratorTuple {
    pub g1: <BoxedDatagram as Datagram>::G,
    pub g2: <BoxedDatagram as Datagram>::G,
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
        parallel: bool,
    ) -> Result<Self::G, Self::Error> {
        Ok(DOperationGeneratorTuple {
            g1: self.d1.operation_generator(geometry, parallel)?,
            g2: self.d2.operation_generator(geometry, parallel)?,
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
