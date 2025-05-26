use std::num::NonZeroU16;

use autd3_core::{common::Hz, sampling_config::SamplingConfig};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SamplingConfigTag {
    Divide = 0,
    Frequency = 1,
    Period = 2,
    FrequencyNearest = 3,
    PeriodNearest = 4,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union SamplingConfigValue {
    pub divide: u16,
    pub freq: f32,
    pub period_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SamplingConfigWrap {
    pub tag: SamplingConfigTag,
    pub value: SamplingConfigValue,
}

impl PartialEq for SamplingConfigWrap {
    fn eq(&self, other: &Self) -> bool {
        SamplingConfig::from(*self) == SamplingConfig::from(*other)
    }
}

impl std::fmt::Debug for SamplingConfigWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self.tag {
                SamplingConfigTag::Divide => {
                    write!(f, "{:?}", NonZeroU16::new_unchecked(self.value.divide))
                }
                SamplingConfigTag::Frequency => write!(f, "{:?}", self.value.freq * Hz),
                SamplingConfigTag::Period => write!(
                    f,
                    "{:?}",
                    std::time::Duration::from_nanos(self.value.period_ns)
                ),
                SamplingConfigTag::FrequencyNearest => {
                    write!(f, "{:?}", self.value.freq * Hz)
                }
                SamplingConfigTag::PeriodNearest => {
                    write!(
                        f,
                        "{:?}",
                        std::time::Duration::from_nanos(self.value.period_ns)
                    )
                }
            }
        }
    }
}

impl From<SamplingConfig> for SamplingConfigWrap {
    fn from(value: SamplingConfig) -> Self {
        match value {
            SamplingConfig::Divide(div) => SamplingConfigWrap {
                tag: SamplingConfigTag::Divide,
                value: SamplingConfigValue { divide: div.get() },
            },
            SamplingConfig::Freq(freq) => SamplingConfigWrap {
                tag: SamplingConfigTag::Frequency,
                value: SamplingConfigValue { freq: freq.hz() },
            },
            SamplingConfig::Period(period) => SamplingConfigWrap {
                tag: SamplingConfigTag::Period,
                value: SamplingConfigValue {
                    period_ns: period.as_nanos() as u64,
                },
            },
            SamplingConfig::FreqNearest(freq) => SamplingConfigWrap {
                tag: SamplingConfigTag::FrequencyNearest,
                value: SamplingConfigValue { freq: freq.0.hz() },
            },
            SamplingConfig::PeriodNearest(period) => SamplingConfigWrap {
                tag: SamplingConfigTag::PeriodNearest,
                value: SamplingConfigValue {
                    period_ns: period.0.as_nanos() as u64,
                },
            },
        }
    }
}

impl From<SamplingConfigWrap> for SamplingConfig {
    fn from(value: SamplingConfigWrap) -> Self {
        unsafe {
            match value.tag {
                SamplingConfigTag::Divide => {
                    SamplingConfig::new(NonZeroU16::new_unchecked(value.value.divide))
                }
                SamplingConfigTag::Frequency => SamplingConfig::new(value.value.freq * Hz),
                SamplingConfigTag::Period => {
                    SamplingConfig::new(std::time::Duration::from_nanos(value.value.period_ns))
                }
                SamplingConfigTag::FrequencyNearest => {
                    SamplingConfig::new(value.value.freq * Hz).into_nearest()
                }
                SamplingConfigTag::PeriodNearest => {
                    SamplingConfig::new(std::time::Duration::from_nanos(value.value.period_ns))
                        .into_nearest()
                }
            }
        }
    }
}
