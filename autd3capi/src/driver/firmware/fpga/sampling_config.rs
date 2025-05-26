use std::num::NonZeroU16;

use autd3capi_driver::{Duration, ResultDuration};
use autd3capi_driver::{
    ResultF32, ResultSamplingConfig, ResultU16, SamplingConfigWrap,
    autd3::core::{derive::SamplingConfigError, sampling_config::SamplingConfig},
    driver::common::Hz,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromDivide(div: u16) -> ResultSamplingConfig {
    match NonZeroU16::new(div) {
        Some(div) => Result::<_, SamplingConfigError>::Ok(SamplingConfig::new(div)).into(),
        None => Result::<SamplingConfig, _>::Err(SamplingConfigError::DivideInvalid).into(),
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFreq(f: f32) -> SamplingConfigWrap {
    SamplingConfig::new(f * Hz).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriod(p: Duration) -> SamplingConfigWrap {
    SamplingConfig::new(std::time::Duration::from(p)).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigIntoNearest(
    config: SamplingConfigWrap,
) -> SamplingConfigWrap {
    SamplingConfig::from(config).into_nearest().into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigDivide(c: SamplingConfigWrap) -> ResultU16 {
    SamplingConfig::from(c).divide().into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFreq(c: SamplingConfigWrap) -> ResultF32 {
    SamplingConfig::from(c).freq().map(|f| f.hz()).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigPeriod(c: SamplingConfigWrap) -> ResultDuration {
    SamplingConfig::from(c).period().into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigEq(
    a: SamplingConfigWrap,
    b: SamplingConfigWrap,
) -> bool {
    SamplingConfig::from(a) == SamplingConfig::from(b)
}
