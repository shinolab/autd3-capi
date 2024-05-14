use autd3capi_driver::{driver::ethercat::DcSysTime as RawDcSysTime, DcSysTime};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDcSysTimeNow() -> DcSysTime {
    RawDcSysTime::now().into()
}
