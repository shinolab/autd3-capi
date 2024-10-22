use autd3::derive::{AUTDInternalError, Datagram, Geometry};
use autd3_driver::firmware::operation::OperationGenerator;

use crate::{take, ConstPtr, DynamicDatagram, DynamicOperationGenerator};

#[repr(C)]
pub struct DatagramPtr(pub *const libc::c_void);

unsafe impl Send for DatagramPtr {}
unsafe impl Sync for DatagramPtr {}

impl DatagramPtr {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub const NULL: Self = Self(std::ptr::null());
}

impl From<DatagramPtr> for Box<DynamicDatagram> {
    fn from(value: DatagramPtr) -> Self {
        unsafe { take!(value, DynamicDatagram) }
    }
}

impl<G: OperationGenerator + 'static, D: Datagram<G = G> + 'static> From<D> for DatagramPtr {
    fn from(d: D) -> Self {
        let d = std::rc::Rc::new(std::cell::RefCell::new(Some(d)));
        let d = DynamicDatagram {
            timeout: Box::new({
                let d = d.clone();
                move || d.borrow().as_ref().unwrap().timeout()
            }),
            parallel_threshold: Box::new({
                let d = d.clone();
                move || d.borrow().as_ref().unwrap().parallel_threshold()
            }),
            g: Box::new(move |geometry: &Geometry| {
                Ok(DynamicOperationGenerator::new(
                    d.borrow_mut()
                        .take()
                        .unwrap()
                        .operation_generator(geometry)?,
                ))
            }),
        };
        Self(Box::into_raw(Box::new(d)) as _)
    }
}

#[repr(C)]

pub struct ResultDatagram {
    pub result: DatagramPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl<G: OperationGenerator + 'static, D: Datagram<G = G> + 'static>
    From<Result<D, AUTDInternalError>> for ResultDatagram
{
    fn from(r: Result<D, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v.into(),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: DatagramPtr::NULL,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}
