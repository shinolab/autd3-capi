#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SamplingConfig {
    pub(crate) div: u32,
}
