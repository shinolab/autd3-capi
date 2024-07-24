use std::time::Duration;

use autd3capi_driver::{
    driver::{defined::Hz, firmware::fpga::STMConfig},
    ResultF32, ResultSamplingConfigWrap, ResultU64, STMConfigWrap, SamplingConfigWrap,
};

pub mod foci;
pub mod gain;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromFreq(f: f32) -> STMConfigWrap {
    STMConfig::Freq(f * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromFreqNearest(f: f32) -> STMConfigWrap {
    STMConfig::FreqNearest(f * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromPeriod(p: u64) -> STMConfigWrap {
    STMConfig::Period(Duration::from_nanos(p)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromPeriodNearest(p: u64) -> STMConfigWrap {
    STMConfig::PeriodNearest(Duration::from_nanos(p)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMConfigFromSamplingConfig(c: SamplingConfigWrap) -> STMConfigWrap {
    STMConfig::SamplingConfig(c.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFreq(c: STMConfigWrap, n: u32) -> ResultF32 {
    STMConfig::from(c)
        .sampling(n as _)
        .and_then(|c| c.freq())
        .map(|f| f.hz() / n as f32)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPeriod(c: STMConfigWrap, n: u32) -> ResultU64 {
    STMConfig::from(c)
        .sampling(n as _)
        .and_then(|c| c.period())
        .map(|f| (f * n).as_nanos() as u64)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMSamplingSamplingConfig(
    c: STMConfigWrap,
    n: u32,
) -> ResultSamplingConfigWrap {
    STMConfig::from(c).sampling(n as _).into()
}
