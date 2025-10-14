use autd3capi_driver::driver::ethercat::DcSysTime;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDcSysTimeNew(sys_time: u64) -> DcSysTime {
    DcSysTime::new(sys_time)
}
