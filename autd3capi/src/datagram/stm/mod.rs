use std::time::Duration;

use autd3capi_driver::{
    driver::{defined::Hz, firmware::fpga::STMSamplingConfig},
    ResultF32, ResultSamplingConfigWrap, ResultU64, STMSamplingConfigWrap, SamplingConfigWrap,
};

pub mod foci;
pub mod gain;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMSamplingConfigFromFreq(f: f32) -> STMSamplingConfigWrap {
    STMSamplingConfig::Freq(f * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMSamplingConfigFromFreqNearest(f: f32) -> STMSamplingConfigWrap {
    STMSamplingConfig::FreqNearest(f * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMSamplingConfigFromPeriod(p: u64) -> STMSamplingConfigWrap {
    STMSamplingConfig::Period(Duration::from_nanos(p)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMSamplingConfigFromPeriodNearest(p: u64) -> STMSamplingConfigWrap {
    STMSamplingConfig::PeriodNearest(Duration::from_nanos(p)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMSamplingConfigFromSamplingConfig(
    c: SamplingConfigWrap,
) -> STMSamplingConfigWrap {
    STMSamplingConfig::SamplingConfig(c.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFreq(c: STMSamplingConfigWrap, n: u32) -> ResultF32 {
    STMSamplingConfig::from(c)
        .sampling(n as _)
        .and_then(|c| c.freq())
        .map(|f| f.hz() / n as f32)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPeriod(c: STMSamplingConfigWrap, n: u32) -> ResultU64 {
    STMSamplingConfig::from(c)
        .sampling(n as _)
        .and_then(|c| c.period())
        .map(|f| (f * n).as_nanos() as u64)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMSamplingSamplingConfig(
    c: STMSamplingConfigWrap,
    n: u32,
) -> ResultSamplingConfigWrap {
    STMSamplingConfig::from(c).sampling(n as _).into()
}
