use autd3capi_driver::driver::ethercat::DcSysTime;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDcSysTimeNow() -> DcSysTime {
    DcSysTime::now()
}
