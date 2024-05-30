use autd3::derive::AUTDInternalError;
use autd3_driver::datagram::{FocusSTM, GainSTM};

use crate::{ConstPtr, G};

#[repr(C)]
pub struct FocusSTMPtr(pub ConstPtr);

impl From<FocusSTM> for FocusSTMPtr {
    fn from(stm: FocusSTM) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}

#[repr(C)]
pub struct GainSTMPtr(pub ConstPtr);

impl From<GainSTM<Box<G>>> for GainSTMPtr {
    fn from(stm: GainSTM<Box<G>>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}

#[repr(C)]
pub struct ResultFocusSTM {
    pub result: FocusSTMPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<FocusSTM, AUTDInternalError>> for ResultFocusSTM {
    fn from(r: Result<FocusSTM, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v.into(),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: FocusSTMPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[repr(C)]
pub struct ResultGainSTM {
    pub result: GainSTMPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<GainSTM<Box<G>>, AUTDInternalError>> for ResultGainSTM {
    fn from(r: Result<GainSTM<Box<G>>, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v.into(),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: GainSTMPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}
