use std::time::Duration;

use autd3capi_driver::{
    autd3::derive::{Datagram, Geometry},
    driver::error::AUTDInternalError,
    DatagramPtr, DynamicDatagram, DynamicOperationGenerator,
};

#[derive(Debug)]
pub struct DynamicDatagramWithParallelThreshold {
    pub d: Box<DynamicDatagram>,
    pub parallel_threshold: Option<usize>,
}

impl Datagram for DynamicDatagramWithParallelThreshold {
    fn operation_generator(self, geometry: &Geometry) -> Result<Self::G, AUTDInternalError> {
        self.d.operation_generator(geometry)
    }

    fn timeout(&self) -> Option<Duration> {
        self.d.timeout()
    }

    fn parallel_threshold(&self) -> Option<usize> {
        self.parallel_threshold
    }

    type G = DynamicOperationGenerator;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramWithParallelThreshold(
    d: DatagramPtr,
    threshold: i32,
) -> DatagramPtr {
    DynamicDatagramWithParallelThreshold {
        d: d.into(),
        parallel_threshold: if threshold < 0 {
            None
        } else {
            Some(threshold as usize)
        },
    }
    .into()
}
