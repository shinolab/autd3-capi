use std::time::Duration;

use autd3_driver::{
    datagram::{Clear, Datagram},
    error::AUTDInternalError,
    operation::Operation,
};

use crate::DynamicDatagram;

impl DynamicDatagram for Clear {
    #[allow(clippy::box_default)]
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::new(<Self as Datagram>::O1::default()),
            Box::new(<Self as Datagram>::O2::default()),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
