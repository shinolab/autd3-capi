#![allow(clippy::missing_safety_doc)]

pub mod focus;
pub mod gain;

use autd3capi_def::{driver::datagram::STMProps, *};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct STMPropsPtr(pub ConstPtr);

impl_ptr!(STMPropsPtr, STMProps);

impl From<STMProps> for STMPropsPtr {
    fn from(value: STMProps) -> Self {
        Self(Box::into_raw(Box::new(value)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFromFreq(freq: float) -> STMPropsPtr {
    STMProps::from_freq(freq).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFromPeriod(p: u64) -> STMPropsPtr {
    STMProps::from_period(std::time::Duration::from_nanos(p)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFromSamplingConfig(
    config: SamplingConfiguration,
) -> STMPropsPtr {
    STMProps::from_sampling_config(config.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithLoopBehavior(
    props: STMPropsPtr,
    loop_behavior: LoopBehavior,
) -> STMPropsPtr {
    take!(props, STMProps)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFrequency(props: STMPropsPtr, size: u64) -> float {
    take!(props, STMProps).freq(size as usize)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsPeriod(props: STMPropsPtr, size: u64) -> u64 {
    take!(props, STMProps).period(size as usize).as_nanos() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsSamplingConfig(
    props: STMPropsPtr,
    size: u64,
) -> ResultSamplingConfig {
    take!(props, STMProps).sampling_config(size as usize).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsLoopBehavior(props: STMPropsPtr) -> LoopBehavior {
    take!(props, STMProps).loop_behavior().into()
}
