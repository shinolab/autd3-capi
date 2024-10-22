use autd3::derive::AUTDInternalError;
use autd3_driver::datagram::{BoxedGain, FociSTM, GainSTM};

use crate::ConstPtr;

#[repr(C)]
pub struct FociSTMPtr(pub *const libc::c_void);

impl<const N: usize> From<FociSTM<N>> for FociSTMPtr {
    fn from(stm: FociSTM<N>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}

#[repr(C)]
pub struct GainSTMPtr(pub *const libc::c_void);

impl From<GainSTM<BoxedGain>> for GainSTMPtr {
    fn from(stm: GainSTM<BoxedGain>) -> Self {
        Self(Box::into_raw(Box::new(stm)) as _)
    }
}

#[repr(C)]
pub struct ResultFociSTM {
    pub result: FociSTMPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl<const N: usize> From<Result<FociSTM<N>, AUTDInternalError>> for ResultFociSTM {
    fn from(r: Result<FociSTM<N>, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v.into(),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: FociSTMPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
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

impl From<Result<GainSTM<BoxedGain>, AUTDInternalError>> for ResultGainSTM {
    fn from(r: Result<GainSTM<BoxedGain>, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v.into(),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: GainSTMPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}
