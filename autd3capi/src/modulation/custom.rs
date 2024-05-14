#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    driver::derive::EmitIntensity, vec_from_raw, CustomModulation, LoopBehavior, ModulationPtr,
    SamplingConfig,
};

use autd3_driver::derive::*;

#[derive(Modulation)]
pub struct CustomModulation {
    pub buf: Vec<EmitIntensity>,
    pub config: SamplingConfig,
    pub loop_behavior: LoopBehavior,
}

impl autd3_driver::datagram::Modulation for CustomModulation {
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustom(
    config: SamplingConfig,
    ptr: *const u8,
    len: u64,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    CustomModulation {
        config: config.into(),
        buf: vec_from_raw!(ptr, EmitIntensity, len),
        loop_behavior: loop_behavior.into(),
    }
    .into()
}
