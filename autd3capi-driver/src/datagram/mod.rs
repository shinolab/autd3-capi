mod tuple;

pub use tuple::DynamicDatagramTuple;

use std::time::Duration;

use autd3::derive::{AUTDInternalError, Geometry};
use autd3_driver::{
    datagram::Datagram,
    firmware::operation::{Operation, OperationGenerator},
    geometry::Device,
};

#[allow(clippy::type_complexity)]
pub struct DynamicOperationGenerator {
    pub g: Box<dyn FnMut(&Device) -> (Box<dyn Operation>, Box<dyn Operation>)>,
}

impl DynamicOperationGenerator {
    pub fn new<
        O1: Operation + 'static,
        O2: Operation + 'static,
        G: OperationGenerator<O1 = O1, O2 = O2> + 'static,
    >(
        mut g: G,
    ) -> Self {
        Self {
            g: Box::new(move |device| {
                let (o1, o2) = g.generate(device);
                (Box::new(o1), Box::new(o2))
            }),
        }
    }
}

impl OperationGenerator for DynamicOperationGenerator {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&mut self, device: &Device) -> (Self::O1, Self::O2) {
        (self.g)(device)
    }
}

#[allow(clippy::type_complexity)]
pub struct DynamicDatagram {
    pub dbg: Box<dyn Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>,
    pub g: Box<dyn FnOnce(&Geometry) -> Result<DynamicOperationGenerator, AUTDInternalError>>,
    pub timeout: Box<dyn Fn() -> Option<Duration>>,
    pub parallel_threshold: Box<dyn Fn() -> Option<usize>>,
}

impl std::fmt::Debug for DynamicDatagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.dbg)(f)
    }
}

unsafe impl Send for DynamicDatagram {}
unsafe impl Sync for DynamicDatagram {}

impl DynamicDatagram {
    pub fn new<G: OperationGenerator + 'static, D: Datagram<G = G> + 'static>(d: D) -> Self {
        let d = std::rc::Rc::new(std::cell::RefCell::new(Some(d)));
        DynamicDatagram {
            dbg: Box::new({
                let d = d.clone();
                move |f| d.borrow().as_ref().unwrap().fmt(f)
            }),
            timeout: Box::new({
                let d = d.clone();
                move || d.borrow().as_ref().unwrap().timeout()
            }),
            parallel_threshold: Box::new({
                let d = d.clone();
                move || d.borrow().as_ref().unwrap().parallel_threshold()
            }),
            g: Box::new(move |geometry: &Geometry| {
                Ok(DynamicOperationGenerator::new(
                    d.borrow_mut()
                        .take()
                        .unwrap()
                        .operation_generator(geometry)?,
                ))
            }),
        }
    }
}

impl Datagram for DynamicDatagram {
    type G = DynamicOperationGenerator;

    fn operation_generator(self, geometry: &Geometry) -> Result<Self::G, AUTDInternalError> {
        (self.g)(geometry)
    }

    fn timeout(&self) -> Option<Duration> {
        (self.timeout)()
    }

    fn parallel_threshold(&self) -> Option<usize> {
        (self.parallel_threshold)()
    }
}
