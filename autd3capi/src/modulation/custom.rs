use autd3capi_driver::{
    autd3::{
        core::modulation::SamplingConfig,
        modulation::{self, CustomOption},
        prelude::Hz,
    },
    vec_from_raw, DynSincInterpolator, ModulationPtr,
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
        option: CustomOption::default(),
    }
    .into()
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustomWithResample(
    ptr: *const u8,
    len: u16,
    src: f32,
    target: SamplingConfig,
    resample: DynSincInterpolator,
) -> ModulationPtr {
    modulation::Custom {
        buffer: vec_from_raw!(ptr, u8, len),
        sampling_config: src * Hz,
        option: CustomOption::default(),
    }
    .with_resample(target, resample)
    .into()
}
