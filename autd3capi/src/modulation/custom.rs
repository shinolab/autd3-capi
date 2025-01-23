use autd3capi_driver::{
    autd3::{core::modulation::SamplingConfig, modulation},
    vec_from_raw, ModulationPtr,
};

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustom(
    ptr: *const u8,
    len: u16,
    sampling_config: SamplingConfig,
) -> ModulationPtr {
    modulation::Custom {
        buffer: vec_from_raw!(ptr, u8, len),
        sampling_config,
    }
    .into()
}
