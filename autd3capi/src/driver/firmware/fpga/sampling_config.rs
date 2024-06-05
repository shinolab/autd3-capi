use autd3capi_driver::{autd3::derive::SamplingConfig, driver::defined::Hz, SamplingConfigWrap};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromDivision(div: u32) -> SamplingConfigWrap {
    SamplingConfig::Division(div).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromDivisionRaw(div: u32) -> SamplingConfigWrap {
    SamplingConfig::DivisionRaw(div).into()
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
