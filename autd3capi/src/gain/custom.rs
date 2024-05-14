use autd3capi_def::*;

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

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainCustom() -> GainPtr {
    CustomGain::default().into()
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
    take_gain!(custom, CustomGain)
        .set(
            dev_idx as _,
            vec_from_raw!(ptr, autd3capi_def::driver::common::Drive, len),
        )
        .into()
}
