use std::time::Duration;

use autd3_driver::defined::Hz;

use crate::SamplingConfigWrap;

#[repr(u8)]
enum STMConfigTag {
    Freq = 1,
    FreqNearest = 2,
    Period = 3,
    PeriodNearest = 4,
    SamplingConfig = 5,
}

#[repr(C)]
union STMConfigValue {
    freq: f32,
    period_ns: u64,
    sampling_config: SamplingConfigWrap,
}

#[repr(C)]
pub struct STMConfigWrap {
    tag: STMConfigTag,
    value: STMConfigValue,
}

impl From<STMConfigWrap> for STMConfig {
    fn from(c: STMConfigWrap) -> Self {
        unsafe {
            match c.tag {
                STMConfigTag::Freq => {
                    autd3_driver::firmware::fpga::STMConfig::Freq(c.value.freq * Hz)
                }
                STMConfigTag::FreqNearest => {
                    autd3_driver::firmware::fpga::STMConfig::FreqNearest(c.value.freq * Hz)
                }
                STMConfigTag::Period => autd3_driver::firmware::fpga::STMConfig::Period(
                    Duration::from_nanos(c.value.period_ns),
                ),
                STMConfigTag::PeriodNearest => {
                    autd3_driver::firmware::fpga::STMConfig::PeriodNearest(Duration::from_nanos(
                        c.value.period_ns,
                    ))
                }
                STMConfigTag::SamplingConfig => {
                    autd3_driver::firmware::fpga::STMConfig::SamplingConfig(
                        c.value.sampling_config.into(),
                    )
                }
            }
        }
    }
}

impl From<autd3_driver::firmware::fpga::STMConfig> for STMConfigWrap {
    fn from(value: autd3_driver::firmware::fpga::STMConfig) -> Self {
        match value {
            STMConfig::Freq(c) => STMConfigWrap {
                tag: STMConfigTag::Freq,
                value: STMConfigValue { freq: c.hz() },
            },
            STMConfig::FreqNearest(c) => STMConfigWrap {
                tag: STMConfigTag::FreqNearest,
                value: STMConfigValue { freq: c.hz() },
            },
            STMConfig::Period(c) => STMConfigWrap {
                tag: STMConfigTag::Period,
                value: STMConfigValue {
                    period_ns: c.as_nanos() as u64,
                },
            },
            STMConfig::PeriodNearest(c) => STMConfigWrap {
                tag: STMConfigTag::PeriodNearest,
                value: STMConfigValue {
                    period_ns: c.as_nanos() as u64,
                },
            },
            STMConfig::SamplingConfig(c) => STMConfigWrap {
                tag: STMConfigTag::SamplingConfig,
                value: STMConfigValue {
                    sampling_config: c.into(),
                },
            },
            _ => unimplemented!(),
        }
    }
}
