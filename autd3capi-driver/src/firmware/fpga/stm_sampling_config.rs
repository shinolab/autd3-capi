use std::time::Duration;

use autd3::derive::{AUTDInternalError, SamplingConfig};
use autd3_driver::datagram::{STMConfig, STMConfigNearest};
use autd3_driver::defined::Hz;

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
    sampling_config: SamplingConfig,
}

#[repr(C)]
pub struct STMConfigWrap {
    tag: STMConfigTag,
    value: STMConfigValue,
}

impl From<STMConfig> for STMConfigWrap {
    fn from(c: STMConfig) -> Self {
        match c {
            STMConfig::Freq(f) => Self {
                tag: STMConfigTag::Freq,
                value: STMConfigValue { freq: f.hz() },
            },
            STMConfig::Period(p) => Self {
                tag: STMConfigTag::Period,
                value: STMConfigValue {
                    period_ns: p.as_nanos() as u64,
                },
            },
            STMConfig::SamplingConfig(c) => Self {
                tag: STMConfigTag::SamplingConfig,
                value: STMConfigValue {
                    sampling_config: c.into(),
                },
            },
            _ => unimplemented!(),
        }
    }
}

impl From<STMConfigNearest> for STMConfigWrap {
    fn from(c: STMConfigNearest) -> Self {
        match c {
            STMConfigNearest::Freq(f) => Self {
                tag: STMConfigTag::FreqNearest,
                value: STMConfigValue { freq: f.hz() },
            },
            STMConfigNearest::Period(p) => Self {
                tag: STMConfigTag::PeriodNearest,
                value: STMConfigValue {
                    period_ns: p.as_nanos() as u64,
                },
            },
            _ => unimplemented!(),
        }
    }
}

impl From<STMConfigWrap> for STMConfig {
    fn from(c: STMConfigWrap) -> Self {
        unsafe {
            match c.tag {
                STMConfigTag::Freq => STMConfig::Freq(c.value.freq * Hz),
                STMConfigTag::Period => STMConfig::Period(Duration::from_nanos(c.value.period_ns)),
                STMConfigTag::SamplingConfig => {
                    STMConfig::SamplingConfig(c.value.sampling_config.into())
                }
                _ => unimplemented!(),
            }
        }
    }
}

impl From<STMConfigWrap> for STMConfigNearest {
    fn from(c: STMConfigWrap) -> Self {
        unsafe {
            match c.tag {
                STMConfigTag::FreqNearest => STMConfigNearest::Freq(c.value.freq * Hz),
                STMConfigTag::PeriodNearest => {
                    STMConfigNearest::Period(Duration::from_nanos(c.value.period_ns))
                }
                _ => unimplemented!(),
            }
        }
    }
}

impl STMConfigWrap {
    pub unsafe fn sampling(self, n: usize) -> Result<SamplingConfig, AUTDInternalError> {
        match self.tag {
            STMConfigTag::Freq | STMConfigTag::Period | STMConfigTag::SamplingConfig => {
                SamplingConfig::try_from((STMConfig::from(self), n))
            }
            STMConfigTag::FreqNearest | STMConfigTag::PeriodNearest => {
                SamplingConfig::try_from((STMConfigNearest::from(self), n))
            }
        }
    }
}
