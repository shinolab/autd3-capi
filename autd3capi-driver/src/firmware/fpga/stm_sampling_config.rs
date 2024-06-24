use std::time::Duration;

use autd3_driver::{defined::Hz, firmware::fpga::STMSamplingConfig};

use crate::SamplingConfigWrap;

#[repr(u8)]
enum STMSamplingConfigTag {
    Freq = 1,
    FreqNearest = 2,
    Period = 3,
    PeriodNearest = 4,
    SamplingConfig = 5,
}

#[repr(C)]
union SamplingConfigValue {
    freq: f32,
    period_ns: u64,
    sampling_config: SamplingConfigWrap,
}

#[repr(C)]
pub struct STMSamplingConfigWrap {
    tag: STMSamplingConfigTag,
    value: SamplingConfigValue,
}

impl From<STMSamplingConfigWrap> for autd3_driver::firmware::fpga::STMSamplingConfig {
    fn from(c: STMSamplingConfigWrap) -> Self {
        unsafe {
            match c.tag {
                STMSamplingConfigTag::Freq => {
                    autd3_driver::firmware::fpga::STMSamplingConfig::Freq(c.value.freq * Hz)
                }
                STMSamplingConfigTag::FreqNearest => {
                    autd3_driver::firmware::fpga::STMSamplingConfig::FreqNearest(c.value.freq * Hz)
                }
                STMSamplingConfigTag::Period => {
                    autd3_driver::firmware::fpga::STMSamplingConfig::Period(Duration::from_nanos(
                        c.value.period_ns,
                    ))
                }
                STMSamplingConfigTag::PeriodNearest => {
                    autd3_driver::firmware::fpga::STMSamplingConfig::PeriodNearest(
                        Duration::from_nanos(c.value.period_ns),
                    )
                }
                STMSamplingConfigTag::SamplingConfig => {
                    autd3_driver::firmware::fpga::STMSamplingConfig::SamplingConfig(
                        c.value.sampling_config.into(),
                    )
                }
            }
        }
    }
}

impl From<autd3_driver::firmware::fpga::STMSamplingConfig> for STMSamplingConfigWrap {
    fn from(value: autd3_driver::firmware::fpga::STMSamplingConfig) -> Self {
        match value {
            STMSamplingConfig::Freq(c) => STMSamplingConfigWrap {
                tag: STMSamplingConfigTag::Freq,
                value: SamplingConfigValue { freq: c.hz() },
            },
            STMSamplingConfig::FreqNearest(c) => STMSamplingConfigWrap {
                tag: STMSamplingConfigTag::FreqNearest,
                value: SamplingConfigValue { freq: c.hz() },
            },
            STMSamplingConfig::Period(c) => STMSamplingConfigWrap {
                tag: STMSamplingConfigTag::Period,
                value: SamplingConfigValue {
                    period_ns: c.as_nanos() as u64,
                },
            },
            STMSamplingConfig::PeriodNearest(c) => STMSamplingConfigWrap {
                tag: STMSamplingConfigTag::PeriodNearest,
                value: SamplingConfigValue {
                    period_ns: c.as_nanos() as u64,
                },
            },
            STMSamplingConfig::SamplingConfig(c) => STMSamplingConfigWrap {
                tag: STMSamplingConfigTag::SamplingConfig,
                value: SamplingConfigValue {
                    sampling_config: c.into(),
                },
            },
            _ => unimplemented!(),
        }
    }
}
