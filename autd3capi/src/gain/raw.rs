use std::collections::HashMap;

use autd3capi_driver::{driver::derive::*, take_gain, vec_from_raw, GainPtr, G};

#[derive(Gain, Default)]
pub struct RawGain {
    drives: HashMap<usize, Vec<Drive>>,
}

impl RawGain {
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

impl Gain for RawGain {
    fn calc(
        &self,
        _geometry: &Geometry,
        _filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(self.drives.clone())
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainCustom() -> GainPtr {
    RawGain::default().into()
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainCustomSet(
    custom: GainPtr,
    dev_idx: u32,
    ptr: *const Drive,
    len: u32,
) -> GainPtr {
    take_gain!(custom, RawGain)
        .set(
            dev_idx as _,
            vec_from_raw!(ptr, autd3capi_driver::driver::firmware::fpga::Drive, len),
        )
        .into()
}
