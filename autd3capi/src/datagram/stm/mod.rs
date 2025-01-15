#[cfg(not(feature = "dynamic_freq"))]
use autd3capi_driver::Duration;
use autd3capi_driver::{
    autd3::core::modulation::SamplingConfig,
    driver::{
        datagram::{IntoSamplingConfigSTM, STMConfig, STMConfigNearest},
        defined::Hz,
    },
    ResultSamplingConfig,
};

pub mod foci;
pub mod gain;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromFreq(f: f32, n: u16) -> ResultSamplingConfig {
    STMConfig::Freq(f * Hz)
        .into_sampling_config(n as usize)
        .into()
}

#[cfg(not(feature = "dynamic_freq"))]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromPeriod(p: Duration, n: u16) -> ResultSamplingConfig {
    STMConfig::Period(p.into())
        .into_sampling_config(n as usize)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromFreqNearest(f: f32, n: u16) -> ResultSamplingConfig {
    STMConfigNearest::Freq(f * Hz)
        .into_sampling_config(n as usize)
        .into()
}

#[cfg(not(feature = "dynamic_freq"))]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromPeriodNearest(
    p: Duration,
    n: u16,
) -> ResultSamplingConfig {
    STMConfigNearest::Period(p.into())
        .into_sampling_config(n as usize)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFreq(c: SamplingConfig, n: u16) -> f32 {
    c.freq().hz() / n as f32
}

#[cfg(not(feature = "dynamic_freq"))]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPeriod(c: SamplingConfig, n: u16) -> Duration {
    (c.period() * n as u32).into()
}
