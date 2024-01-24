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
pub unsafe extern "C" fn AUTDSTMPropsWithStartIdx(props: STMPropsPtr, idx: i32) -> STMPropsPtr {
    if idx < 0 {
        take!(props, STMProps).with_start_idx(None)
    } else {
        take!(props, STMProps).with_start_idx(Some(idx as u16))
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithFinishIdx(props: STMPropsPtr, idx: i32) -> STMPropsPtr {
    if idx < 0 {
        take!(props, STMProps).with_finish_idx(None)
    } else {
        take!(props, STMProps).with_finish_idx(Some(idx as u16))
    }
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
pub unsafe extern "C" fn AUTDSTMPropsStartIdx(props: STMPropsPtr) -> i32 {
    if let Some(idx) = props.start_idx() {
        idx as i32
    } else {
        -1
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFinishIdx(props: STMPropsPtr) -> i32 {
    if let Some(idx) = props.finish_idx() {
        idx as i32
    } else {
        -1
    }
}
