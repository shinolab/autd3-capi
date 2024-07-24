use autd3::prelude::*;
use autd3_driver::error::AUTDInternalError;

use crate::{ConstPtr, SamplingConfigTag, SamplingConfigValue, SamplingConfigWrap};

pub const AUTD3_ERR: i32 = -1;
pub const AUTD3_TRUE: i32 = 1;
pub const AUTD3_FALSE: i32 = 0;

#[repr(C)]

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
            err: ConstPtr(std::ptr::null_mut()),
        }
    }
}

impl From<bool> for ResultI32 {
    fn from(v: bool) -> Self {
        Self {
            result: if v { AUTD3_TRUE } else { AUTD3_FALSE },
            err_len: 0,
            err: ConstPtr(std::ptr::null_mut()),
        }
    }
}

impl From<usize> for ResultI32 {
    fn from(v: usize) -> Self {
        Self {
            result: v as i32,
            err_len: 0,
            err: ConstPtr(std::ptr::null_mut()),
        }
    }
}

impl From<AUTDInternalError> for ResultI32 {
    fn from(e: AUTDInternalError) -> Self {
        let err = e.to_string();
        Self {
            result: AUTD3_ERR,
            err_len: err.as_bytes().len() as u32 + 1,
            err: ConstPtr(Box::into_raw(Box::new(err)) as _),
        }
    }
}

impl From<AUTDError> for ResultI32 {
    fn from(e: AUTDError) -> Self {
        let err = e.to_string();
        Self {
            result: AUTD3_ERR,
            err_len: err.as_bytes().len() as u32 + 1,
            err: ConstPtr(Box::into_raw(Box::new(err)) as _),
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

pub struct ResultU16 {
    pub result: u16,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<u16, AUTDInternalError>> for ResultU16 {
    fn from(r: Result<u16, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v,
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: 0,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

#[repr(C)]

pub struct ResultF32 {
    pub result: f32,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<f32, AUTDInternalError>> for ResultF32 {
    fn from(r: Result<f32, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v,
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: 0.,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

#[repr(C)]

pub struct ResultU64 {
    pub result: u64,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<u64, AUTDInternalError>> for ResultU64 {
    fn from(r: Result<u64, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v,
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: 0,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

#[repr(C)]
pub struct ResultSamplingConfigWrap {
    pub result: SamplingConfigWrap,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<SamplingConfig, AUTDInternalError>> for ResultSamplingConfigWrap {
    fn from(r: Result<SamplingConfig, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v.into(),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: SamplingConfigWrap {
                        tag: SamplingConfigTag::Division,
                        value: SamplingConfigValue { div: 0 },
                    },
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}
