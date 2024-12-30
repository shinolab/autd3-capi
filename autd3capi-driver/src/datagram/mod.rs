mod tuple;

pub use tuple::DynDatagramTuple;

use std::{mem::MaybeUninit, time::Duration};

use autd3::derive::{AUTDDriverError, Geometry};
use autd3_driver::{
    datagram::Datagram,
    firmware::operation::{Operation, OperationGenerator},
    geometry::Device,
};

pub trait DOperationGenerator {
    fn dyn_generate(&mut self, device: &Device) -> (Box<dyn Operation>, Box<dyn Operation>);
}

pub struct DynOperationGenerator {
    g: Box<dyn DOperationGenerator>,
}

impl OperationGenerator for DynOperationGenerator {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&mut self, device: &Device) -> (Self::O1, Self::O2) {
        self.g.dyn_generate(device)
    }
}

impl<G: OperationGenerator> DOperationGenerator for G
where
    G::O1: 'static,
    G::O2: 'static,
{
    fn dyn_generate(&mut self, device: &Device) -> (Box<dyn Operation>, Box<dyn Operation>) {
        let (o1, o2) = self.generate(device);
        (Box::new(o1), Box::new(o2))
    }
}

pub trait DDatagram: std::fmt::Debug {
    fn dyn_operation_generator(
        &mut self,
        geometry: &Geometry,
    ) -> Result<Box<dyn DOperationGenerator>, AUTDDriverError>;
    fn dyn_timeout(&self) -> Option<Duration>;
    fn dyn_parallel_threshold(&self) -> Option<usize>;
    fn dyn_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<G: DOperationGenerator + 'static, T: Datagram<G = G>> DDatagram for MaybeUninit<T> {
    fn dyn_operation_generator(
        &mut self,
        geometry: &Geometry,
    ) -> Result<Box<dyn DOperationGenerator>, AUTDDriverError> {
        let mut tmp = MaybeUninit::<T>::uninit();
        std::mem::swap(&mut tmp, self);
        let d = unsafe { tmp.assume_init() };
        Ok(Box::new(d.operation_generator(geometry)?))
    }

    fn dyn_timeout(&self) -> Option<Duration> {
        unsafe { self.assume_init_ref() }.timeout()
    }

    fn dyn_parallel_threshold(&self) -> Option<usize> {
        unsafe { self.assume_init_ref() }.parallel_threshold()
    }

    fn dyn_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { self.assume_init_ref() }.fmt(f)
    }
}

pub struct DynDatagram {
    d: Box<dyn DDatagram>,
}

impl std::fmt::Debug for DynDatagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.d.dyn_fmt(f)
    }
}

unsafe impl Send for DynDatagram {}
unsafe impl Sync for DynDatagram {}

impl DynDatagram {
    pub fn new<G: OperationGenerator + 'static, D: Datagram<G = G> + 'static>(d: D) -> Self {
        DynDatagram {
            d: Box::new(MaybeUninit::new(d)),
        }
    }
}

impl Datagram for DynDatagram {
    type G = DynOperationGenerator;

    fn operation_generator(self, geometry: &Geometry) -> Result<Self::G, AUTDDriverError> {
        let Self { mut d } = self;
        Ok(DynOperationGenerator {
            g: d.dyn_operation_generator(geometry)?,
        })
    }

    fn timeout(&self) -> Option<Duration> {
        self.d.dyn_timeout()
    }

    fn parallel_threshold(&self) -> Option<usize> {
        self.d.dyn_parallel_threshold()
    }
}
