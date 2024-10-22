mod gain;
mod modulation;

pub use gain::*;
pub use modulation::*;

use std::time::Duration;

use autd3::derive::{AUTDInternalError, Geometry};
use autd3_driver::{
    datagram::Datagram,
    firmware::operation::{Operation, OperationGenerator},
    geometry::Device,
};

use derive_more::Debug;

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

#[derive(Debug)]
#[allow(clippy::type_complexity)]
pub struct DynamicDatagram {
    #[debug(skip)]
    pub g: Box<dyn FnOnce(&Geometry) -> Result<DynamicOperationGenerator, AUTDInternalError>>,
    #[debug(skip)]
    pub timeout: Box<dyn Fn() -> Option<Duration>>,
    #[debug(skip)]
    pub parallel_threshold: Box<dyn Fn() -> Option<usize>>,
}

unsafe impl Send for DynamicDatagram {}
unsafe impl Sync for DynamicDatagram {}

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
