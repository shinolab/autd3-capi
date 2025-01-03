use autd3capi_driver::{
    autd3::prelude::IntoDatagramWithParallelThreshold, DatagramPtr, DynDatagram,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramWithParallelThreshold(
    d: DatagramPtr,
    threshold: i32,
) -> DatagramPtr {
    Box::<DynDatagram>::from(d)
        .with_parallel_threshold(if threshold < 0 {
            None
        } else {
            Some(threshold as usize)
        })
        .into()
}
