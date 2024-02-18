use std::time::Duration;

use autd3_driver::{
    datagram::{Datagram, Synchronize},
    error::AUTDInternalError,
    operation::Operation,
};

use crate::DynamicDatagram;

impl DynamicDatagram for Synchronize {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::<autd3_driver::operation::SyncOp>::default(),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
