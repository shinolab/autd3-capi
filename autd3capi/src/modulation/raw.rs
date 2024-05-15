#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{driver::derive::*, take, vec_from_raw, ModulationPtr, SamplingConfigPtr};

#[derive(Modulation)]
pub struct RawModulation {
    pub buf: Vec<u8>,
    pub config: SamplingConfig,
    pub loop_behavior: LoopBehavior,
}

impl Modulation for RawModulation {
    fn calc(&self, _geometry: &Geometry) -> Result<Vec<u8>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationRaw(
    config: SamplingConfigPtr,
    loop_behavior: autd3capi_driver::LoopBehavior,
    ptr: *const u8,
    len: u32,
) -> ModulationPtr {
    RawModulation {
        config: *take!(config, _),
        buf: vec_from_raw!(ptr, u8, len),
        loop_behavior: loop_behavior.into(),
    }
    .into()
}
