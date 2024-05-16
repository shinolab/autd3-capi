use autd3_driver::defined::Hz;

#[repr(u8)]
enum SamplingConfigTag {
    Division,
    DivisionRaw,
    Freq,
    FreqNearest,
}

#[repr(C)]
pub struct SamplingConfigWrap {
    ty: SamplingConfigTag,
    value_uint: u32,
    value_float: f64,
}

impl From<SamplingConfigWrap> for autd3_driver::firmware::fpga::SamplingConfig {
    fn from(c: SamplingConfigWrap) -> Self {
        match c.ty {
            SamplingConfigTag::Division => {
                autd3_driver::firmware::fpga::SamplingConfig::Division(c.value_uint)
            }
            SamplingConfigTag::DivisionRaw => {
                autd3_driver::firmware::fpga::SamplingConfig::DivisionRaw(c.value_uint)
            }
            SamplingConfigTag::Freq => {
                autd3_driver::firmware::fpga::SamplingConfig::Freq(c.value_uint * Hz)
            }
            SamplingConfigTag::FreqNearest => {
                autd3_driver::firmware::fpga::SamplingConfig::FreqNearest(c.value_float * Hz)
            }
        }
    }
}

impl From<autd3_driver::firmware::fpga::SamplingConfig> for SamplingConfigWrap {
    fn from(value: autd3_driver::firmware::fpga::SamplingConfig) -> Self {
        match value {
            autd3::derive::SamplingConfig::Freq(c) => SamplingConfigWrap {
                ty: SamplingConfigTag::Freq,
                value_uint: c.hz(),
                value_float: 0.0,
            },
            autd3::derive::SamplingConfig::FreqNearest(c) => SamplingConfigWrap {
                ty: SamplingConfigTag::FreqNearest,
                value_uint: 0,
                value_float: c.hz(),
            },
            autd3::derive::SamplingConfig::DivisionRaw(c) => SamplingConfigWrap {
                ty: SamplingConfigTag::DivisionRaw,
                value_uint: c,
                value_float: 0.0,
            },
            autd3::derive::SamplingConfig::Division(c) => SamplingConfigWrap {
                ty: SamplingConfigTag::Division,
                value_uint: c,
                value_float: 0.0,
            },
        }
    }
}
