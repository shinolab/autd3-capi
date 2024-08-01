use std::{num::NonZeroU16, time::Duration};

use autd3capi_driver::{autd3::derive::SamplingConfig, driver::defined::Hz, ResultSamplingConfig};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromDivision(div: u16) -> SamplingConfig {
    SamplingConfig::new(NonZeroU16::new_unchecked(div))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreq(f: u32) -> ResultSamplingConfig {
    (f * Hz).try_into().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreqF(f: f32) -> ResultSamplingConfig {
    (f * Hz).try_into().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreqNearest(f: f32) -> SamplingConfig {
    SamplingConfig::new_nearest(f * Hz)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriod(p: u64) -> ResultSamplingConfig {
    Duration::from_nanos(p).try_into().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriodNearest(p: u64) -> SamplingConfig {
    SamplingConfig::new_nearest(Duration::from_nanos(p))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigDivision(c: SamplingConfig) -> u16 {
    c.division()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFreq(c: SamplingConfig) -> f32 {
    c.freq().hz()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigPeriod(c: SamplingConfig) -> u64 {
    c.period().as_nanos() as u64
}
