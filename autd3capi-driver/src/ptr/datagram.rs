use autd3_core::datagram::{Datagram, Operation};
use autd3_driver::{
    datagram::BoxedDatagram, error::AUTDDriverError, firmware::operation::OperationGenerator,
};

use crate::{impl_ptr, take};

#[repr(C)]
pub struct DatagramPtr(pub *const libc::c_void);

impl_ptr!(DatagramPtr);

impl From<DatagramPtr> for Box<BoxedDatagram> {
    fn from(value: DatagramPtr) -> Self {
        unsafe { take!(value, BoxedDatagram) }
    }
}

impl<E, G: OperationGenerator + 'static, D: Datagram<G = G, Error = E> + 'static> From<D>
    for DatagramPtr
where
    AUTDDriverError: From<E>,
    AUTDDriverError: From<<G::O1 as Operation>::Error> + From<<G::O2 as Operation>::Error>,
{
    fn from(d: D) -> Self {
        use autd3_driver::datagram::IntoBoxedDatagram;
        Self(Box::into_raw(Box::new(d.into_boxed())) as _)
    }
}
