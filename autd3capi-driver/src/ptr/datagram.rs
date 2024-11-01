use autd3::derive::Datagram;
use autd3_driver::firmware::operation::OperationGenerator;

use crate::{impl_ffi_result, impl_ptr, take, ConstPtr, DynDatagram};

#[repr(C)]
pub struct DatagramPtr(pub *const libc::c_void);

impl_ptr!(DatagramPtr);

impl From<DatagramPtr> for Box<DynDatagram> {
    fn from(value: DatagramPtr) -> Self {
        unsafe { take!(value, DynDatagram) }
    }
}

#[repr(C)]
pub struct ResultDatagram {
    pub result: DatagramPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl<G: OperationGenerator + 'static, D: Datagram<G = G> + 'static> From<D> for DatagramPtr {
    fn from(d: D) -> Self {
        Self(Box::into_raw(Box::new(DynDatagram::new(d))) as _)
    }
}

impl_ffi_result!(ResultDatagram, DatagramPtr);
