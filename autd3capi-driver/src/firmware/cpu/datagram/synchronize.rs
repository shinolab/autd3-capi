use std::time::Duration;

use autd3_driver::{
    datagram::{Datagram, Synchronize},
    firmware::operation::Operation,
};

use crate::DynamicDatagram;

impl DynamicDatagram for Synchronize {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        (
            Box::<autd3_driver::firmware::operation::SyncOp>::default(),
            Box::<autd3_driver::firmware::operation::NullOp>::default(),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
