use autd3capi_driver::{DatagramPtr, DynDatagramTuple, driver::firmware::driver::BoxedDatagram};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramTuple(d1: DatagramPtr, d2: DatagramPtr) -> DatagramPtr {
    DynDatagramTuple {
        d1: *Box::<BoxedDatagram>::from(d1),
        d2: *Box::<BoxedDatagram>::from(d2),
    }
    .into()
}
