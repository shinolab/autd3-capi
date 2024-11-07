use autd3capi_driver::{
    autd3::derive::SamplingConfig, driver::defined::Hz, Duration, ResultSamplingConfig,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromDivision(div: u16) -> ResultSamplingConfig {
    SamplingConfig::new(div).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreq(f: u32) -> ResultSamplingConfig {
    SamplingConfig::new(f * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreqF(f: f32) -> ResultSamplingConfig {
    SamplingConfig::new(f * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreqNearest(f: f32) -> SamplingConfig {
    SamplingConfig::new_nearest(f * Hz)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriod(p: Duration) -> ResultSamplingConfig {
    std::time::Duration::from(p).try_into().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriodNearest(p: Duration) -> SamplingConfig {
    SamplingConfig::new_nearest(std::time::Duration::from(p))
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
