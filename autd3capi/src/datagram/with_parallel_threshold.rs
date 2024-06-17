use std::time::Duration;

use autd3capi_driver::{
    autd3::derive::{Geometry, Operation},
    driver::{derive::tracing, error::AUTDInternalError, firmware::operation::OperationGenerator},
    DatagramPtr, DynamicDatagram,
};

pub struct DynamicDatagramWithParallelThreshold {
    pub d: Box<Box<dyn DynamicDatagram>>,
    pub parallel_threshold: usize,
}

impl DynamicDatagram for DynamicDatagramWithParallelThreshold {
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
        self.d.timeout()
    }

    fn parallel_threshold(&self) -> Option<usize> {
        Some(self.parallel_threshold)
    }

    #[tracing::instrument(skip(self, geometry))]
    fn trace(&self, geometry: &Geometry) {
        tracing::info!(
            "DynamicDatagramWithParallelThreshold ({:?})",
            self.parallel_threshold
        );
        self.d.trace(geometry)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramWithParallelThreshold(
    d: DatagramPtr,
    threshold: u16,
) -> DatagramPtr {
    DynamicDatagramWithParallelThreshold {
        d: d.into(),
        parallel_threshold: threshold as usize,
    }
    .into()
}
