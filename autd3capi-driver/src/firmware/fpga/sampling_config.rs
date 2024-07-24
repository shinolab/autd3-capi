use std::{num::NonZeroU16, time::Duration};

use autd3_driver::defined::Hz;

#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum SamplingConfigTag {
    Division = 0,
    Freq = 2,
    FreqNearest = 3,
    Period = 4,
    PeriodNearest = 5,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) union SamplingConfigValue {
    pub(crate) div: u16,
    pub(crate) freq: u32,
    pub(crate) freq_nearest: f32,
    pub(crate) period_ns: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplingConfigWrap {
    pub(crate) tag: SamplingConfigTag,
    pub(crate) value: SamplingConfigValue,
}

impl From<SamplingConfigWrap> for autd3_driver::firmware::fpga::SamplingConfig {
    fn from(c: SamplingConfigWrap) -> Self {
        unsafe {
            match c.tag {
                SamplingConfigTag::Division => {
                    autd3_driver::firmware::fpga::SamplingConfig::Division(
                        NonZeroU16::new_unchecked(c.value.div),
                    )
                }
                SamplingConfigTag::Freq => {
                    autd3_driver::firmware::fpga::SamplingConfig::Freq(c.value.freq * Hz)
                }
                SamplingConfigTag::FreqNearest => {
                    autd3_driver::firmware::fpga::SamplingConfig::FreqNearest(
                        c.value.freq_nearest * Hz,
                    )
                }
                SamplingConfigTag::Period => autd3_driver::firmware::fpga::SamplingConfig::Period(
                    Duration::from_nanos(c.value.period_ns),
                ),
                SamplingConfigTag::PeriodNearest => {
                    autd3_driver::firmware::fpga::SamplingConfig::PeriodNearest(
                        Duration::from_nanos(c.value.period_ns),
                    )
                }
            }
        }
    }
}

impl From<autd3_driver::firmware::fpga::SamplingConfig> for SamplingConfigWrap {
    fn from(value: autd3_driver::firmware::fpga::SamplingConfig) -> Self {
        match value {
            autd3::derive::SamplingConfig::Freq(c) => SamplingConfigWrap {
                tag: SamplingConfigTag::Freq,
                value: SamplingConfigValue { freq: c.hz() },
            },
            autd3::derive::SamplingConfig::FreqNearest(c) => SamplingConfigWrap {
                tag: SamplingConfigTag::FreqNearest,
                value: SamplingConfigValue {
                    freq_nearest: c.hz(),
                },
            },
            autd3::derive::SamplingConfig::Division(c) => SamplingConfigWrap {
                tag: SamplingConfigTag::Division,
                value: SamplingConfigValue { div: c.get() },
            },
            autd3::derive::SamplingConfig::Period(c) => SamplingConfigWrap {
                tag: SamplingConfigTag::Period,
                value: SamplingConfigValue {
                    period_ns: c.as_nanos() as u64,
                },
            },
            autd3::derive::SamplingConfig::PeriodNearest(c) => SamplingConfigWrap {
                tag: SamplingConfigTag::PeriodNearest,
                value: SamplingConfigValue {
                    period_ns: c.as_nanos() as u64,
                },
            },
            _ => unimplemented!(),
        }
    }
}
