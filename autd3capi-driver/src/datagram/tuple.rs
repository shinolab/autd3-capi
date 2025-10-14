use autd3::{
    core::{
        datagram::{Datagram, DeviceMask},
        environment::Environment,
        geometry::Geometry,
    },
    driver::{
        error::AUTDDriverError,
        firmware::operation::{BoxedDatagram, BoxedOperation, OperationGenerator},
        geometry::Device,
    },
};

pub struct DynDatagramTuple {
    pub d1: BoxedDatagram,
    pub d2: BoxedDatagram,
}

pub struct DOperationGeneratorTuple {
    pub g1: <BoxedDatagram as Datagram<'static>>::G,
    pub g2: <BoxedDatagram as Datagram<'static>>::G,
}

impl OperationGenerator<'static> for DOperationGeneratorTuple {
    type O1 = BoxedOperation;
    type O2 = BoxedOperation;

    fn generate(&mut self, device: &Device) -> Option<(Self::O1, Self::O2)> {
        match (self.g1.generate(device), self.g2.generate(device)) {
            (Some((o1, _)), Some((o2, _))) => Some((o1, o2)),
            _ => None,
        }
    }
}

impl Datagram<'static> for DynDatagramTuple {
    type G = DOperationGeneratorTuple;
    type Error = AUTDDriverError;

    fn operation_generator(
        self,
        geometry: &Geometry,
        env: &Environment,
        filter: &DeviceMask,
    ) -> Result<Self::G, Self::Error> {
        Ok(DOperationGeneratorTuple {
            g1: self.d1.operation_generator(geometry, env, filter)?,
            g2: self.d2.operation_generator(geometry, env, filter)?,
        })
    }

    fn option(&self) -> autd3::core::datagram::DatagramOption {
        self.d1.option().merge(self.d2.option())
    }
}
