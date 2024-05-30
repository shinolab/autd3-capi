use autd3::derive::AUTDInternalError;

use crate::{take, ConstPtr, DynamicDatagram};

#[repr(C)]
pub struct DatagramPtr(pub ConstPtr);

impl DatagramPtr {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub const NULL: Self = Self(std::ptr::null());
}

impl From<DatagramPtr> for Box<Box<dyn DynamicDatagram>> {
    fn from(value: DatagramPtr) -> Self {
        unsafe { take!(value, Box<dyn DynamicDatagram>) }
    }
}

impl<T: DynamicDatagram> From<T> for DatagramPtr {
    fn from(d: T) -> Self {
        let d: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(d));
        Self(Box::into_raw(d) as _)
    }
}

#[repr(C)]

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
