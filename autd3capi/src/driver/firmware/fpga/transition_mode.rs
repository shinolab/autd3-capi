use autd3capi_driver::{
    autd3::derive::TransitionMode,
    driver::ethercat::{DcSysTime, ECAT_DC_SYS_TIME_BASE},
    GPIOIn, TransitionModeWrap,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSyncIdx() -> TransitionModeWrap {
    TransitionMode::SyncIdx.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSysTime(sys_time: u64) -> TransitionModeWrap {
    TransitionMode::SysTime(
        DcSysTime::from_utc(ECAT_DC_SYS_TIME_BASE + std::time::Duration::from_nanos(sys_time))
            .unwrap(),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeGPIO(gpio: GPIOIn) -> TransitionModeWrap {
    TransitionMode::GPIO(gpio.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeExt() -> TransitionModeWrap {
    TransitionMode::Ext.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeImmediate() -> TransitionModeWrap {
    TransitionMode::Immediate.into()
}
