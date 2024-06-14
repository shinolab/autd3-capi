pub mod clear;
pub mod debug;
pub mod force_fan;
pub mod pulse_width_encoder;
pub mod reads_fpga_state;
pub mod segment;
pub mod silencer;
pub mod stm;
pub mod synchronize;
pub mod with_parallel_threshold;
pub mod with_timeout;

use std::time::Duration;

use autd3capi_driver::{
    autd3::derive::{Device, Geometry, Operation},
    driver::{error::AUTDInternalError, firmware::operation::OperationGenerator},
    DatagramPtr, DynamicDatagram,
};

pub struct DynamicDatagramTuple {
    pub d1: Box<Box<dyn DynamicDatagram>>,
    pub d2: Box<Box<dyn DynamicDatagram>>,
}

pub struct DynamicOperationGeneratorTuple {
    pub g1: Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
    pub g2: Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
}

impl OperationGenerator for DynamicOperationGeneratorTuple {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&self, device: &Device) -> (Self::O1, Self::O2) {
        (self.g1.generate(device).0, self.g2.generate(device).0)
    }
}

impl DynamicDatagram for DynamicDatagramTuple {
    fn operation_generator(
        &mut self,
        geometry: &Geometry,
    ) -> Result<
        Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
        AUTDInternalError,
    > {
        Ok(Box::new(DynamicOperationGeneratorTuple {
            g1: self.d1.operation_generator(geometry)?,
            g2: self.d2.operation_generator(geometry)?,
        }))
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

    #[tracing::instrument(skip(self, geometry))]
    fn trace(&self, geometry: &Geometry) {
        self.d1.trace(geometry);
        self.d2.trace(geometry);
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramTuple(d1: DatagramPtr, d2: DatagramPtr) -> DatagramPtr {
    DynamicDatagramTuple {
        d1: d1.into(),
        d2: d2.into(),
    }
    .into()
}
