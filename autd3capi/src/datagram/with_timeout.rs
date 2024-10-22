use std::time::Duration;

use autd3capi_driver::{
    autd3::derive::{Datagram, Geometry},
    driver::error::AUTDInternalError,
    DatagramPtr, DynamicDatagram, DynamicOperationGenerator,
};

#[derive(Debug)]
pub struct DynamicDatagramWithTimeout {
    pub d: Box<DynamicDatagram>,
    pub timeout: Option<Duration>,
}

impl Datagram for DynamicDatagramWithTimeout {
    fn operation_generator(self, geometry: &Geometry) -> Result<Self::G, AUTDInternalError> {
        self.d.operation_generator(geometry)
    }

    fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    fn parallel_threshold(&self) -> Option<usize> {
        self.d.parallel_threshold()
    }

    type G = DynamicOperationGenerator;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramWithTimeout(d: DatagramPtr, timeout_ns: i64) -> DatagramPtr {
    DynamicDatagramWithTimeout {
        d: d.into(),
        timeout: if timeout_ns < 0 {
            None
        } else {
            Some(Duration::from_nanos(timeout_ns as u64))
        },
    }
    .into()
}
