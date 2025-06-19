use autd3::{
    core::{
        datagram::{Datagram, DeviceFilter, FirmwareLimits},
        geometry::Geometry,
    },
    driver::{
        error::AUTDDriverError,
        firmware::{
            auto::operation::OperationGenerator,
            driver::{BoxedDatagram, BoxedOperation},
        },
        geometry::Device,
    },
    firmware::Version,
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

    fn generate(&mut self, device: &Device, version: Version) -> Option<(Self::O1, Self::O2)> {
        match (
            self.g1.generate(device, version),
            self.g2.generate(device, version),
        ) {
            (Some((o1, _)), Some((o2, _))) => Some((o1, o2)),
            _ => None,
        }
    }
}

impl Datagram for DynDatagramTuple {
    type G = DOperationGeneratorTuple;
    type Error = AUTDDriverError;

    fn operation_generator(
        self,
        geometry: &Geometry,
        filter: &DeviceFilter,
        limits: &FirmwareLimits,
    ) -> Result<Self::G, Self::Error> {
        Ok(DOperationGeneratorTuple {
            g1: self.d1.operation_generator(geometry, filter, limits)?,
            g2: self.d2.operation_generator(geometry, filter, limits)?,
        })
    }

    fn option(&self) -> autd3::core::datagram::DatagramOption {
        self.d1.option().merge(self.d2.option())
    }
}
