use std::time::Duration;

use autd3_driver::{
    datagram::{Clear, Datagram},
    firmware::operation::Operation,
};

use crate::DynamicDatagram;

impl DynamicDatagram for Clear {
    #[allow(clippy::box_default)]
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        (
            Box::new(<Self as Datagram>::O1::default()),
            Box::new(<Self as Datagram>::O2::default()),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
