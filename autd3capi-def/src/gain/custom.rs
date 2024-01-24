use std::collections::HashMap;

use autd3_driver::derive::*;

#[derive(Gain, Default)]
pub struct CustomGain {
    drives: HashMap<usize, Vec<Drive>>,
}

impl CustomGain {
    pub fn new() -> Self {
        Self {
            drives: Default::default(),
        }
    }

    pub fn set(self, dev_idx: usize, drives: Vec<Drive>) -> Self {
        let mut new = self;
        new.drives.insert(dev_idx, drives);
        new
    }
}

impl autd3_driver::datagram::Gain for CustomGain {
    fn calc(
        &self,
        _geometry: &Geometry,
        _filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(self.drives.clone())
    }
}
