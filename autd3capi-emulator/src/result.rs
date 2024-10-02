use autd3_emulator::{EmulatorError, Record, SoundField};
use autd3capi_driver::{ConstPtr, AUTD3_ERR, AUTD3_TRUE};

use crate::{RecordPtr, SoundFieldPtr};

#[repr(C)]
pub struct ResultRecord {
    pub result: RecordPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<Record, EmulatorError>> for ResultRecord {
    fn from(r: Result<Record, EmulatorError>) -> Self {
        match r {
            Ok(v) => Self {
                result: RecordPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: RecordPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

#[repr(C)]

pub struct ResultEmualtorErr {
    pub result: i32,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<(), EmulatorError>> for ResultEmualtorErr {
    fn from(value: Result<(), EmulatorError>) -> Self {
        match value {
            Ok(_) => Self {
                result: AUTD3_TRUE,
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: AUTD3_ERR,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

#[repr(C)]

pub struct ResultSoundField {
    pub result: SoundFieldPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<SoundField<'static>, EmulatorError>> for ResultSoundField {
    fn from(value: Result<SoundField<'static>, EmulatorError>) -> Self {
        match value {
            Ok(v) => Self {
                result: SoundFieldPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: SoundFieldPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}
