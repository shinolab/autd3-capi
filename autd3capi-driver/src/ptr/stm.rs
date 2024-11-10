use autd3::prelude::ControlPoints;
use autd3_driver::datagram::{BoxedGain, FociSTM, GainSTM};

use crate::{impl_ffi_result, impl_ptr, ConstPtr};

#[repr(C)]
pub struct FociSTMPtr(pub *const libc::c_void);

impl_ptr!(FociSTMPtr);

impl<const N: usize> From<FociSTM<N, Vec<ControlPoints<N>>>> for FociSTMPtr {
    fn from(stm: FociSTM<N, Vec<ControlPoints<N>>>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}

#[repr(C)]
pub struct GainSTMPtr(pub *const libc::c_void);

impl_ptr!(GainSTMPtr);

impl From<GainSTM<Vec<BoxedGain>>> for GainSTMPtr {
    fn from(stm: GainSTM<Vec<BoxedGain>>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}

#[repr(C)]
pub struct ResultFociSTM {
    pub result: FociSTMPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_ffi_result!(ResultFociSTM, FociSTMPtr);

#[repr(C)]
pub struct ResultGainSTM {
    pub result: GainSTMPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_ffi_result!(ResultGainSTM, GainSTMPtr);
