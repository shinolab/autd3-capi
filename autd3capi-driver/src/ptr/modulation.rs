use autd3::derive::AUTDInternalError;
use autd3_driver::derive::Modulation;

use crate::{ConstPtr, M};

#[repr(C)]
pub struct ModulationPtr(pub *const libc::c_void);

impl<T: Modulation + Send + Sync + 'static> From<T> for ModulationPtr {
    fn from(m: T) -> Self {
        let m: Box<Box<M>> = Box::new(Box::new(m));
        Self(Box::into_raw(m) as _)
    }
}

#[macro_export]
macro_rules! take_mod {
    ($ptr:expr, $type:ty) => {
        Box::from_raw($ptr.0 as *mut Box<M> as *mut Box<$type>)
    };
}

#[repr(C)]

pub struct ResultModulation {
    pub result: ModulationPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl<T: Modulation + Send + Sync + 'static> From<Result<T, AUTDInternalError>>
    for ResultModulation
{
    fn from(r: Result<T, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v.into(),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: ModulationPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}
