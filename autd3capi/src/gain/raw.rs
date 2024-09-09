use std::{collections::HashMap, sync::Arc};

use autd3capi_driver::{
    autd3::prelude::Drive, driver::derive::*, take_gain, vec_from_raw, GainPtr, G,
};

#[derive(Gain, Default, Debug)]
pub struct RawGain {
    drives: HashMap<usize, Arc<Vec<Drive>>>,
}

impl RawGain {
    pub fn new() -> Self {
        Self {
            drives: Default::default(),
        }
    }

    pub fn set(self, dev_idx: usize, drives: Vec<Drive>) -> Self {
        let mut new = self;
        new.drives.insert(dev_idx, Arc::new(drives));
        new
    }
}

impl Gain for RawGain {
    fn calc(&self, _geometry: &Geometry) -> Result<GainCalcFn, AUTDInternalError> {
        let drives = &self.drives;
        Ok(Box::new(|dev| {
            let drives = drives[&dev.idx()].clone();
            Box::new(move |tr| drives[tr.idx()])
        }))
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainRaw() -> GainPtr {
    RawGain::default().into()
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainRawSet(
    custom: GainPtr,
    dev_idx: u16,
    ptr: *const Drive,
    len: u8,
) -> GainPtr {
    take_gain!(custom, RawGain)
        .set(
            dev_idx as _,
            vec_from_raw!(ptr, autd3capi_driver::driver::firmware::fpga::Drive, len),
        )
        .into()
}
