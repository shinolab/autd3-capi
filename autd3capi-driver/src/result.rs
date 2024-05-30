use crate::ConstPtr;
use autd3::prelude::*;
use autd3_driver::error::AUTDInternalError;

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
