#[cfg(not(feature = "dynamic_freq"))]
use autd3capi_driver::Duration;
use autd3capi_driver::{
    autd3::core::modulation::SamplingConfig, driver::defined::Hz, ResultSamplingConfig,
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

#[cfg(not(feature = "dynamic_freq"))]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriod(p: Duration) -> ResultSamplingConfig {
    std::time::Duration::from(p).try_into().into()
}

#[cfg(not(feature = "dynamic_freq"))]
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

#[cfg(not(feature = "dynamic_freq"))]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigPeriod(c: SamplingConfig) -> Duration {
    c.period().into()
}
