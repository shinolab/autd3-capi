use std::time::Duration;

use autd3_driver::{
    datagram::{Datagram, PulseWidthEncoder},
    firmware::operation::Operation,
};

use crate::DynamicDatagram;

impl DynamicDatagram for PulseWidthEncoder {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        (
            Box::new(<<Self as Datagram>::O1>::new(self.buf().to_vec())),
            Box::<<Self as Datagram>::O2>::default(),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
