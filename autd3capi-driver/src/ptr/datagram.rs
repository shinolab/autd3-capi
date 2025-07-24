use autd3::{
    core::datagram::Datagram,
    driver::{
        error::AUTDDriverError,
        firmware::{
            auto::operation::OperationGenerator,
            driver::{BoxedDatagram, Operation},
        },
    },
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

impl<E, G: OperationGenerator<'static> + 'static, D: Datagram<'static, G = G, Error = E> + 'static>
    From<D> for DatagramPtr
where
    AUTDDriverError: From<E>,
    AUTDDriverError:
        From<<G::O1 as Operation<'static>>::Error> + From<<G::O2 as Operation<'static>>::Error>,
{
    fn from(d: D) -> Self {
        Self(Box::into_raw(Box::new(BoxedDatagram::new(d))) as _)
    }
}
