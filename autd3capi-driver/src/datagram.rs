use std::time::Duration;

use autd3::derive::{AUTDInternalError, Geometry};
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
}

unsafe impl Send for DynamicDatagramPack {}
unsafe impl Sync for DynamicDatagramPack {}

pub struct DynamicOperationGeneratorPack2 {
    pub g1: Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
    pub g2: Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
}

impl OperationGenerator for DynamicOperationGeneratorPack2 {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&self, device: &autd3::derive::Device) -> (Self::O1, Self::O2) {
        (self.g1.generate(device).0, self.g2.generate(device).0)
    }
}

pub struct DynamicDatagramPack2 {
    pub d1: Box<Box<dyn DynamicDatagram>>,
    pub d2: Box<Box<dyn DynamicDatagram>>,
}

impl Datagram for DynamicDatagramPack2 {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;
    type G = DynamicOperationGeneratorPack2;

    fn operation_generator(mut self, geometry: &Geometry) -> Result<Self::G, AUTDInternalError> {
        Ok(DynamicOperationGeneratorPack2 {
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
}

unsafe impl Send for DynamicDatagramPack2 {}
unsafe impl Sync for DynamicDatagramPack2 {}
