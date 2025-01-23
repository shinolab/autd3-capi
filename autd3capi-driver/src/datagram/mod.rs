mod tuple;

pub use tuple::DynDatagramTuple;

use std::mem::MaybeUninit;

use autd3_core::{
    datagram::{DatagramOption, Operation},
    geometry::Geometry,
};
use autd3_driver::{
    datagram::Datagram,
    error::AUTDDriverError,
    firmware::operation::{BoxedOperation, OperationGenerator},
    geometry::Device,
};

pub trait DOperationGenerator {
    fn dyn_generate(&mut self, device: &Device) -> (BoxedOperation, BoxedOperation);
}

pub struct DynOperationGenerator {
    g: Box<dyn DOperationGenerator>,
}

impl OperationGenerator for DynOperationGenerator {
    type O1 = BoxedOperation;
    type O2 = BoxedOperation;

    fn generate(&mut self, device: &Device) -> (Self::O1, Self::O2) {
        self.g.dyn_generate(device)
    }
}

impl<G: OperationGenerator> DOperationGenerator for G
where
    G::O1: 'static,
    G::O2: 'static,
    AUTDDriverError: From<<G::O1 as Operation>::Error> + From<<G::O2 as Operation>::Error>,
{
    fn dyn_generate(&mut self, device: &Device) -> (BoxedOperation, BoxedOperation) {
        let (o1, o2) = self.generate(device);
        (BoxedOperation::new(o1), BoxedOperation::new(o2))
    }
}

pub trait DDatagram: std::fmt::Debug {
    fn dyn_operation_generator(
        &mut self,
        geometry: &Geometry,
        option: &DatagramOption,
    ) -> Result<Box<dyn DOperationGenerator>, AUTDDriverError>;
    fn dyn_option(&self) -> DatagramOption;
    fn dyn_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<E, G: DOperationGenerator + 'static, T: Datagram<G = G, Error = E>> DDatagram
    for MaybeUninit<T>
where
    AUTDDriverError: From<E>,
{
    fn dyn_operation_generator(
        &mut self,
        geometry: &Geometry,
        option: &DatagramOption,
    ) -> Result<Box<dyn DOperationGenerator>, AUTDDriverError> {
        let mut tmp = MaybeUninit::<T>::uninit();
        std::mem::swap(&mut tmp, self);
        let d = unsafe { tmp.assume_init() };
        Ok(Box::new(d.operation_generator(geometry, option)?))
    }

    fn dyn_option(&self) -> DatagramOption {
        unsafe { self.assume_init_ref() }.option()
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
    pub fn new<E, G: OperationGenerator + 'static, D: Datagram<G = G, Error = E> + 'static>(
        d: D,
    ) -> Self
    where
        AUTDDriverError: From<E>,
        AUTDDriverError: From<<G::O1 as Operation>::Error> + From<<G::O2 as Operation>::Error>,
    {
        DynDatagram {
            d: Box::new(MaybeUninit::new(d)),
        }
    }
}

impl Datagram for DynDatagram {
    type G = DynOperationGenerator;
    type Error = AUTDDriverError;

    fn operation_generator(
        self,
        geometry: &Geometry,
        option: &DatagramOption,
    ) -> Result<Self::G, Self::Error> {
        let Self { mut d } = self;
        Ok(DynOperationGenerator {
            g: d.dyn_operation_generator(geometry, option)?,
        })
    }

    fn option(&self) -> DatagramOption {
        self.d.dyn_option()
    }
}
