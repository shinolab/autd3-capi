use autd3capi_driver::{
    autd3::{derive::*, modulation, prelude::Hz},
    DynSincInterpolator, ModulationPtr,
};

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustom(
    config: SamplingConfig,
    loop_behavior: autd3capi_driver::LoopBehavior,
    ptr: *const u8,
    len: u16,
) -> ModulationPtr {
    modulation::Custom::new((0..len as usize).map(|i| ptr.add(i).read()), config)
        .unwrap()
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustomWithResample(
    loop_behavior: autd3capi_driver::LoopBehavior,
    ptr: *const u8,
    len: u16,
    src: f32,
    target: SamplingConfig,
    resample: DynSincInterpolator,
) -> ModulationPtr {
    modulation::Custom::new_with_resample(
        (0..len as usize).map(|i| ptr.add(i).read()),
        src * Hz,
        target,
        resample,
    )
    .unwrap()
    .with_loop_behavior(loop_behavior.into())
    .into()
}
