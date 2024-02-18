use crate::{ConstPtr, DynamicDatagram, G};
use autd3::prelude::*;
use autd3_driver::error::AUTDInternalError;

use crate::{
    DatagramPtr, FocusSTMPtr, GainSTMPtr, ModulationPtr, AUTD3_ERR, AUTD3_FALSE, AUTD3_TRUE,
};

#[cfg(feature = "export")]
mod export {
    use super::*;
    use std::ffi::{c_char, CStr, CString};

    #[no_mangle]
    pub unsafe extern "C" fn AUTDGetErr(src: ConstPtr, dst: *mut c_char) {
        let src = Box::from_raw(src as *mut String);
        let c_string: CString = CString::new(src.as_str()).unwrap();
        let c_str: &CStr = c_string.as_c_str();
        libc::strcpy(dst, c_str.as_ptr());
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultI32 {
    pub result: i32,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<()> for ResultI32 {
    fn from(_: ()) -> Self {
        Self {
            result: AUTD3_TRUE,
            err_len: 0,
            err: std::ptr::null_mut(),
        }
    }
}

impl From<bool> for ResultI32 {
    fn from(v: bool) -> Self {
        Self {
            result: if v { AUTD3_TRUE } else { AUTD3_FALSE },
            err_len: 0,
            err: std::ptr::null_mut(),
        }
    }
}

impl From<usize> for ResultI32 {
    fn from(v: usize) -> Self {
        Self {
            result: v as i32,
            err_len: 0,
            err: std::ptr::null_mut(),
        }
    }
}

impl From<AUTDInternalError> for ResultI32 {
    fn from(e: AUTDInternalError) -> Self {
        let err = e.to_string();
        Self {
            result: AUTD3_ERR,
            err_len: err.as_bytes().len() as u32 + 1,
            err: Box::into_raw(Box::new(err)) as _,
        }
    }
}

impl From<AUTDError> for ResultI32 {
    fn from(e: AUTDError) -> Self {
        let err = e.to_string();
        Self {
            result: AUTD3_ERR,
            err_len: err.as_bytes().len() as u32 + 1,
            err: Box::into_raw(Box::new(err)) as _,
        }
    }
}

impl<T, E> From<Result<T, E>> for ResultI32
where
    T: Into<Self>,
    E: Into<Self>,
{
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(t) => t.into(),
            Err(e) => e.into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultModulation {
    pub result: ModulationPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultDatagram {
    pub result: DatagramPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl<T: DynamicDatagram> From<Result<T, AUTDInternalError>> for ResultDatagram {
    fn from(r: Result<T, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v.into(),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: DatagramPtr::NULL,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
                    result: FocusSTMPtr(std::ptr::null_mut()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
                    result: GainSTMPtr(std::ptr::null_mut()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}
