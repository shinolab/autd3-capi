use autd3capi_driver::{autd3::derive::SamplingConfig, driver::defined::Hz, SamplingConfigPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromDivision(div: u32) -> SamplingConfigPtr {
    SamplingConfig::Division(div).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromDivisionRaw(div: u32) -> SamplingConfigPtr {
    SamplingConfig::DivisionRaw(div).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreq(f: u32) -> SamplingConfigPtr {
    SamplingConfig::Freq(f * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreqNearest(f: f64) -> SamplingConfigPtr {
    SamplingConfig::FreqNearest(f * Hz).into()
}
