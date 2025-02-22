use autd3capi_driver::{ModulationPtr, SamplingConfigWrap, autd3::modulation, vec_from_raw};

#[unsafe(no_mangle)]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustom(
    ptr: *const u8,
    len: u16,
    sampling_config: SamplingConfigWrap,
) -> ModulationPtr {
    modulation::Custom {
        buffer: vec_from_raw!(ptr, u8, len),
        sampling_config,
    }
    .into()
}
