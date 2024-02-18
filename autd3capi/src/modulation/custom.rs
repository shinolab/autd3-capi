#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    driver::derive::EmitIntensity, vec_from_raw, CustomModulation, LoopBehavior, ModulationPtr,
    SamplingConfiguration,
};

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustom(
    config: SamplingConfiguration,
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
