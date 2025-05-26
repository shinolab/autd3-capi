use autd3capi_driver::{
    Duration, ResultSamplingConfig, SamplingConfigWrap,
    driver::{common::Hz, datagram::STMConfig},
};

pub mod foci;
pub mod gain;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromFreq(f: f32, n: u16) -> ResultSamplingConfig {
    STMConfig::Freq(f * Hz)
        .into_sampling_config(n as usize)
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromPeriod(p: Duration, n: u16) -> ResultSamplingConfig {
    STMConfig::Period(p.into())
        .into_sampling_config(n as usize)
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromFreqNearest(f: f32, n: u16) -> SamplingConfigWrap {
    STMConfig::FreqNearest(f * Hz)
        .into_sampling_config(n as usize)
        .unwrap()
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromPeriodNearest(p: Duration, n: u16) -> SamplingConfigWrap {
    STMConfig::PeriodNearest(p.into())
        .into_sampling_config(n as usize)
        .unwrap()
        .into()
}
