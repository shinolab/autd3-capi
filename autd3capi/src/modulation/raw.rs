#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{autd3::derive::*, vec_from_raw, ModulationPtr};

#[derive(Modulation)]
pub struct RawModulation {
    pub buf: Arc<Vec<u8>>,
    pub config: SamplingConfig,
    pub loop_behavior: LoopBehavior,
}

impl Modulation for RawModulation {
    fn calc(&self) -> ModulationCalcResult {
        Ok(self.buf.clone())
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationRaw(
    config: SamplingConfig,
    loop_behavior: autd3capi_driver::LoopBehavior,
    ptr: *const u8,
    len: u16,
) -> ModulationPtr {
    RawModulation {
        config,
        buf: Arc::new(vec_from_raw!(ptr, u8, len)),
        loop_behavior: loop_behavior.into(),
    }
    .into()
}
