use autd3::derive::SamplingConfig;

use crate::ConstPtr;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SamplingConfigPtr(pub ConstPtr);

impl From<SamplingConfig> for SamplingConfigPtr {
    fn from(c: SamplingConfig) -> Self {
        Self(Box::into_raw(Box::new(c)) as _)
    }
}
