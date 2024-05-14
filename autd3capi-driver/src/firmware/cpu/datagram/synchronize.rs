use std::time::Duration;

use autd3_driver::{
    datagram::{Datagram, Synchronize},
    firmware::operation::Operation,
};

use crate::DynamicDatagram;

impl DynamicDatagram for Synchronize {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        (
            Box::<<Self as Datagram>::O1>::default(),
            Box::<<Self as Datagram>::O2>::default(),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
