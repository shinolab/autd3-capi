use std::time::Duration;

use super::DynamicDatagram;
use autd3::derive::{HashMap, DEFAULT_TIMEOUT};
use autd3_driver::firmware::operation::Operation;

pub struct DynamicReadsFPGAState {
    map: HashMap<usize, bool>,
}

impl DynamicReadsFPGAState {
    pub const fn new(map: HashMap<usize, bool>) -> Self {
        Self { map }
    }
}

impl DynamicDatagram for DynamicReadsFPGAState {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        let map = self.map.clone();
        (
            Box::new(autd3_driver::firmware::operation::ReadsFPGAStateOp::new(
                move |dev| map[&dev.idx()],
            )),
            Box::new(autd3_driver::firmware::operation::NullOp::default()),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        Some(DEFAULT_TIMEOUT)
    }
}
