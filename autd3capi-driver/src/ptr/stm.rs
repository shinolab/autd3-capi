use autd3_driver::datagram::{FocusSTM, GainSTM};

use crate::{ConstPtr, G};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FocusSTMPtr(pub ConstPtr);

impl From<FocusSTM> for FocusSTMPtr {
    fn from(stm: FocusSTM) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GainSTMPtr(pub ConstPtr);

impl From<GainSTM<Box<G>>> for GainSTMPtr {
    fn from(stm: GainSTM<Box<G>>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}
