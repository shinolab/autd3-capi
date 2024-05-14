#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFrequencyDivision(div: u32) -> ResultSamplingConfig {
    autd3_driver::common::SamplingConfig::from_frequency_division(div).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFrequency(f: f64) -> ResultSamplingConfig {
    autd3_driver::common::SamplingConfig::from_frequency(f).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriod(p: u64) -> ResultSamplingConfig {
    autd3_driver::common::SamplingConfig::from_period(std::time::Duration::from_nanos(p)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFrequencyDivision(config: SamplingConfig) -> u32 {
    autd3_driver::common::SamplingConfig::from(config).frequency_division()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFrequency(config: SamplingConfig) -> f64 {
    autd3_driver::common::SamplingConfig::from(config).frequency()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigPeriod(config: SamplingConfig) -> u64 {
    autd3_driver::common::SamplingConfig::from(config)
        .period()
        .as_nanos() as _
}
