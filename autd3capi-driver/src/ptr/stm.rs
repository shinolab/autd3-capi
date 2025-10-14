use autd3::{
    core::derive::SamplingConfig,
    driver::datagram::{BoxedGain, ControlPoints, FociSTM, GainSTM},
};

use crate::impl_ptr;

#[repr(C)]
pub struct FociSTMPtr(pub *const std::ffi::c_void);

impl_ptr!(FociSTMPtr);

impl<const N: usize> From<FociSTM<N, Vec<ControlPoints<N>>, SamplingConfig>> for FociSTMPtr {
    fn from(stm: FociSTM<N, Vec<ControlPoints<N>>, SamplingConfig>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}

#[repr(C)]
pub struct GainSTMPtr(pub *const std::ffi::c_void);

impl_ptr!(GainSTMPtr);

impl From<GainSTM<Vec<BoxedGain<'static>>, SamplingConfig>> for GainSTMPtr {
    fn from(stm: GainSTM<Vec<BoxedGain<'static>>, SamplingConfig>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}
