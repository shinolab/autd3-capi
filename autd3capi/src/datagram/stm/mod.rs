use std::time::Duration;

use autd3capi_driver::{
    autd3::derive::SamplingConfig,
    driver::{
        datagram::{STMConfig, STMConfigNearest},
        defined::Hz,
    },
    ResultSamplingConfig,
};

pub mod foci;
pub mod gain;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromFreq(f: f32, n: u16) -> ResultSamplingConfig {
    (STMConfig::Freq(f * Hz), n as usize).try_into().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromPeriod(p: u64, n: u16) -> ResultSamplingConfig {
    (STMConfig::Period(Duration::from_nanos(p)), n as usize)
        .try_into()
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromFreqNearest(f: f32, n: u16) -> ResultSamplingConfig {
    (STMConfigNearest::Freq(f * Hz), n as usize)
        .try_into()
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromPeriodNearest(p: u64, n: u16) -> ResultSamplingConfig {
    (
        STMConfigNearest::Period(Duration::from_nanos(p)),
        n as usize,
    )
        .try_into()
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFreq(c: SamplingConfig, n: u16) -> f32 {
    c.freq().hz() / n as f32
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPeriod(c: SamplingConfig, n: u16) -> u64 {
    (c.period() * n as u32).as_nanos() as u64
}
