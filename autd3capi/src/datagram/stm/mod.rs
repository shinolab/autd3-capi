#![allow(clippy::missing_safety_doc)]

pub mod focus;
pub mod gain;

use autd3capi_driver::{
    driver::{datagram::STMProps, defined::Hz},
    *,
};

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
pub unsafe extern "C" fn AUTDSTMPropsFromFreq(freq: f64) -> STMPropsPtr {
    STMProps::from_freq(freq * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFromFreqNearest(freq: f64) -> STMPropsPtr {
    STMProps::from_freq_nearest(freq * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFromSamplingConfig(config: SamplingConfigPtr) -> STMPropsPtr {
    STMProps::from_sampling_config(*take!(config, _)).into()
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
