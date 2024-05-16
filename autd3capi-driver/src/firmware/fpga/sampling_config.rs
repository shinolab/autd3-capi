use autd3_driver::defined::Hz;

#[repr(u8)]
enum SamplingConfigTag {
    Division,
    DivisionRaw,
    Freq,
    FreqNearest,
}

#[repr(C)]
union SamplingConfigValue {
    div: u32,
    freq: u32,
    freq_nearest: f64,
}

#[repr(C)]
pub struct SamplingConfigWrap {
    tag: SamplingConfigTag,
    value: SamplingConfigValue,
}

impl From<SamplingConfigWrap> for autd3_driver::firmware::fpga::SamplingConfig {
    fn from(c: SamplingConfigWrap) -> Self {
        unsafe {
            match c.tag {
                SamplingConfigTag::Division => {
                    autd3_driver::firmware::fpga::SamplingConfig::Division(c.value.div)
                }
                SamplingConfigTag::DivisionRaw => {
                    autd3_driver::firmware::fpga::SamplingConfig::DivisionRaw(c.value.div)
                }
                SamplingConfigTag::Freq => {
                    autd3_driver::firmware::fpga::SamplingConfig::Freq(c.value.freq * Hz)
                }
                SamplingConfigTag::FreqNearest => {
                    autd3_driver::firmware::fpga::SamplingConfig::FreqNearest(
                        c.value.freq_nearest * Hz,
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
            autd3::derive::SamplingConfig::DivisionRaw(c) => SamplingConfigWrap {
                tag: SamplingConfigTag::DivisionRaw,
                value: SamplingConfigValue { div: c },
            },
            autd3::derive::SamplingConfig::Division(c) => SamplingConfigWrap {
                tag: SamplingConfigTag::Division,
                value: SamplingConfigValue { div: c },
            },
        }
    }
}
