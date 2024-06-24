use std::time::Duration;

use autd3capi_driver::{
    autd3::derive::Geometry,
    driver::{
        derive::tracing,
        error::AUTDInternalError,
        firmware::operation::{Operation, OperationGenerator},
    },
    DatagramPtr, DynamicDatagram,
};

pub struct DynamicDatagramWithTimeout {
    pub d: Box<Box<dyn DynamicDatagram>>,
    pub timeout: Duration,
}

impl DynamicDatagram for DynamicDatagramWithTimeout {
    fn operation_generator(
        &mut self,
        geometry: &Geometry,
    ) -> Result<
        Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
        AUTDInternalError,
    > {
        self.d.operation_generator(geometry)
    }

    fn timeout(&self) -> Option<Duration> {
        Some(self.timeout)
    }

    fn parallel_threshold(&self) -> Option<usize> {
        self.d.parallel_threshold()
    }

    #[tracing::instrument(skip(self, geometry))]
    fn trace(&self, geometry: &Geometry) {
        tracing::info!("DynamicDatagramWithTimeout ({:?})", self.timeout);
        self.d.trace(geometry)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramWithTimeout(d: DatagramPtr, timeout_ns: u64) -> DatagramPtr {
    DynamicDatagramWithTimeout {
        d: d.into(),
        timeout: Duration::from_nanos(timeout_ns),
    }
    .into()
}
