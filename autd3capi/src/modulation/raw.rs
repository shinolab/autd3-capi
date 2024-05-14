#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    driver::derive::*, take, take_mod, vec_from_raw, ModulationPtr, SamplingConfigPtr, M,
};

#[derive(Modulation)]
pub struct RawModulation {
    pub buf: HashMap<usize, Vec<u8>>,
    pub config: SamplingConfig,
    pub loop_behavior: LoopBehavior,
}

impl RawModulation {
    pub fn set(self, dev_idx: usize, drives: Vec<u8>) -> Self {
        let mut new = self;
        new.buf.insert(dev_idx, drives);
        new
    }
}

impl Modulation for RawModulation {
    fn calc(&self, _geometry: &Geometry) -> Result<HashMap<usize, Vec<u8>>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustom(
    config: SamplingConfigPtr,
    loop_behavior: autd3capi_driver::LoopBehavior,
) -> ModulationPtr {
    RawModulation {
        config: *take!(config, _),
        buf: HashMap::default(),
        loop_behavior: loop_behavior.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustomSet(
    custom: ModulationPtr,
    dev_idx: u32,
    ptr: *const u8,
    len: u32,
) -> ModulationPtr {
    take_mod!(custom, RawModulation)
        .set(dev_idx as _, vec_from_raw!(ptr, u8, len))
        .into()
}
