use autd3_core::datagram::{Datagram, Operation};
use autd3_driver::{error::AUTDDriverError, firmware::operation::OperationGenerator};

use crate::{impl_ptr, impl_result, take, ConstPtr, DynDatagram};

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

impl<E, G: OperationGenerator + 'static, D: Datagram<G = G, Error = E> + 'static> From<D>
    for DatagramPtr
where
    AUTDDriverError: From<E>,
    AUTDDriverError: From<<G::O1 as Operation>::Error> + From<<G::O2 as Operation>::Error>,
{
    fn from(d: D) -> Self {
        Self(Box::into_raw(Box::new(DynDatagram::new(d))) as _)
    }
}

impl_result!(ResultDatagram, DatagramPtr);
