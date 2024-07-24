use std::{num::NonZeroU16, time::Duration};

use autd3capi_driver::{
    autd3::derive::SamplingConfig, driver::defined::Hz, ResultF32, ResultU16, ResultU64,
    SamplingConfigWrap,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromDivision(div: u16) -> SamplingConfigWrap {
    SamplingConfig::Division(NonZeroU16::new_unchecked(div)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreq(f: u32) -> SamplingConfigWrap {
    SamplingConfig::Freq(f * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreqNearest(f: f32) -> SamplingConfigWrap {
    SamplingConfig::FreqNearest(f * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriod(p: u64) -> SamplingConfigWrap {
    SamplingConfig::Period(Duration::from_nanos(p)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriodNearest(p: u64) -> SamplingConfigWrap {
    SamplingConfig::PeriodNearest(Duration::from_nanos(p)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigDivision(c: SamplingConfigWrap) -> ResultU16 {
    SamplingConfig::from(c).division().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFreq(c: SamplingConfigWrap) -> ResultF32 {
    SamplingConfig::from(c).freq().map(|f| f.hz()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigPeriod(c: SamplingConfigWrap) -> ResultU64 {
    SamplingConfig::from(c)
        .period()
        .map(|p| p.as_nanos() as u64)
        .into()
}
