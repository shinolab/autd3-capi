use std::time::Duration;

use autd3::derive::{tracing, AUTDInternalError, Geometry};
use autd3_driver::{
    datagram::Datagram,
    firmware::operation::{Operation, OperationGenerator},
};

pub struct DynamicOperationGenerator<O1: Operation + 'static, O2: Operation + 'static> {
    pub g: Box<dyn OperationGenerator<O1 = O1, O2 = O2>>,
}

impl<O1: Operation + 'static, O2: Operation + 'static> OperationGenerator
    for DynamicOperationGenerator<O1, O2>
{
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&self, device: &autd3::derive::Device) -> (Self::O1, Self::O2) {
        let (o1, o2) = self.g.generate(device);
        (Box::new(o1), Box::new(o2))
    }
}

pub trait DynamicDatagram {
    #[allow(clippy::type_complexity)]
    fn operation_generator(
        &mut self,
        geometry: &Geometry,
    ) -> Result<
        Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
        AUTDInternalError,
    >;
    fn timeout(&self) -> Option<Duration>;
    fn parallel_threshold(&self) -> Option<usize>;
    fn trace(&self, geometry: &Geometry);
}

impl<
        O1: Operation + 'static,
        O2: Operation + 'static,
        G: OperationGenerator + 'static,
        D: Datagram<O1 = O1, O2 = O2, G = G> + Default,
    > DynamicDatagram for D
{
    fn operation_generator(
        &mut self,
        geometry: &Geometry,
    ) -> Result<
        Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
        AUTDInternalError,
    > {
        let b: D = std::mem::take(self);
        let g = <Self as Datagram>::operation_generator(b, geometry)?;
        let b = DynamicOperationGenerator { g: Box::new(g) };
        Ok(Box::new(b))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }

    fn parallel_threshold(&self) -> Option<usize> {
        <Self as Datagram>::parallel_threshold(self)
    }

    #[tracing::instrument(skip(self, geometry))]
    fn trace(&self, geometry: &Geometry) {
        <Self as Datagram>::trace(self, geometry)
    }
}

pub struct DynamicOperationGeneratorPack {
    pub g: Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
}

impl OperationGenerator for DynamicOperationGeneratorPack {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&self, device: &autd3::derive::Device) -> (Self::O1, Self::O2) {
        self.g.generate(device)
    }
}

pub struct DynamicDatagramPack {
    pub d: Box<Box<dyn DynamicDatagram>>,
}

impl Datagram for DynamicDatagramPack {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;
    type G = DynamicOperationGeneratorPack;

    fn operation_generator(mut self, geometry: &Geometry) -> Result<Self::G, AUTDInternalError> {
        Ok(DynamicOperationGeneratorPack {
            g: self.d.operation_generator(geometry)?,
        })
    }

    fn timeout(&self) -> Option<Duration> {
        self.d.timeout()
    }

    fn parallel_threshold(&self) -> Option<usize> {
        self.d.parallel_threshold()
    }

    #[tracing::instrument(skip(self, geometry))]
    fn trace(&self, geometry: &Geometry) {
        self.d.trace(geometry)
    }
}

unsafe impl Send for DynamicDatagramPack {}
unsafe impl Sync for DynamicDatagramPack {}
