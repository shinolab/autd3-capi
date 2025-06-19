use autd3::driver::error::AUTDDriverError;

use crate::{
    CapiResult, ConstPtr, Duration, SamplingConfigTag, SamplingConfigValue, SamplingConfigWrap,
};

#[macro_export]
macro_rules! impl_result {
    ($type:ty, $inner:ident) => {
        impl<T, E> From<Result<T, E>> for $type
        where
            $inner: From<T>,
            E: std::error::Error,
        {
            fn from(value: Result<T, E>) -> Self {
                match value {
                    Ok(value) => Self {
                        result: value.into(),
                        err_len: 0,
                        err: ConstPtr(std::ptr::null_mut()),
                    },
                    Err(e) => {
                        tracing::error!("{}", e);
                        let err = e.to_string();
                        Self {
                            result: $inner::NULL,
                            err_len: err.as_bytes().len() as u32 + 1,
                            err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! validate_cstr {
    ($path:expr, $type:tt, $retty:tt) => {
        match unsafe { std::ffi::CStr::from_ptr($path).to_str() } {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("{}", e);
                let err = e.to_string();
                return $retty {
                    result: $type::NULL,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                };
            }
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AUTDStatus {
    AUTDTrue = 0,
    AUTDFalse = 1,
    AUTDErr = 2,
}

impl AUTDStatus {
    pub const NULL: Self = Self::AUTDErr;
}

#[repr(C)]
pub struct ResultStatus {
    pub result: AUTDStatus,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<()> for AUTDStatus {
    fn from(_: ()) -> Self {
        Self::AUTDTrue
    }
}

impl From<AUTDDriverError> for AUTDStatus {
    fn from(_: AUTDDriverError) -> Self {
        Self::AUTDErr
    }
}

impl_result!(ResultStatus, AUTDStatus);

#[repr(C)]
pub struct ResultSamplingConfig {
    pub result: SamplingConfigWrap,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl CapiResult for SamplingConfigWrap {
    const NULL: Self = SamplingConfigWrap {
        tag: SamplingConfigTag::Divide,
        value: SamplingConfigValue { divide: 0 },
    };
}

impl_result!(ResultSamplingConfig, SamplingConfigWrap);

#[repr(C)]
pub struct ResultU8 {
    pub result: u8,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl CapiResult for u8 {
    const NULL: Self = 0;
}

impl_result!(ResultU8, u8);

#[repr(C)]
pub struct ResultU16 {
    pub result: u16,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl CapiResult for u16 {
    const NULL: Self = 0;
}

impl_result!(ResultU16, u16);

#[repr(C)]
pub struct ResultF32 {
    pub result: f32,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl CapiResult for f32 {
    const NULL: Self = 0.;
}

impl_result!(ResultF32, f32);

#[repr(C)]
pub struct ResultDuration {
    pub result: Duration,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl CapiResult for Duration {
    const NULL: Self = Duration { nanos: 0 };
}

impl_result!(ResultDuration, Duration);
