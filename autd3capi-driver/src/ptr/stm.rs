use autd3_core::derive::SamplingConfig;
use autd3_driver::{
    datagram::{BoxedGain, FociSTM, GainSTM},
    firmware::operation::ControlPoints,
};

use crate::impl_ptr;

#[repr(C)]
pub struct FociSTMPtr(pub *const libc::c_void);

impl_ptr!(FociSTMPtr);

impl<const N: usize> From<FociSTM<N, Vec<ControlPoints<N>>, SamplingConfig>> for FociSTMPtr {
    fn from(stm: FociSTM<N, Vec<ControlPoints<N>>, SamplingConfig>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}

#[repr(C)]
pub struct GainSTMPtr(pub *const libc::c_void);

impl_ptr!(GainSTMPtr);

impl From<GainSTM<Vec<BoxedGain>, SamplingConfig>> for GainSTMPtr {
    fn from(stm: GainSTM<Vec<BoxedGain>, SamplingConfig>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}
