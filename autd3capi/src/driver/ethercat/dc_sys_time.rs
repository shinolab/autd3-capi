use autd3capi_driver::driver::ethercat::DcSysTime;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDcSysTimeNow() -> u64 {
    DcSysTime::now().sys_time()
}
