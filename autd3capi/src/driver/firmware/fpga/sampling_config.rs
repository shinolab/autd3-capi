use std::{num::NonZeroU16, time::Duration};

use autd3capi_driver::{
    autd3::derive::{IntoSamplingConfig, IntoSamplingConfigNearest, SamplingConfig},
    driver::defined::Hz,
    ResultSamplingConfig,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromDivision(div: u16) -> SamplingConfig {
    SamplingConfig::new(NonZeroU16::new_unchecked(div)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreq(f: u32) -> ResultSamplingConfig {
    (f * Hz).into_sampling_config().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreqF(f: f32) -> ResultSamplingConfig {
    (f * Hz).into_sampling_config().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreqNearest(f: f32) -> SamplingConfig {
    (f * Hz).into_sampling_config_nearest()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriod(p: u64) -> ResultSamplingConfig {
    Duration::from_nanos(p).into_sampling_config().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriodNearest(p: u64) -> SamplingConfig {
    Duration::from_nanos(p).into_sampling_config_nearest()
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
