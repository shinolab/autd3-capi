use autd3capi_driver::{
    autd3::{derive::TransitionMode, prelude::GPIOIn},
    driver::ethercat::{DcSysTime, ECAT_DC_SYS_TIME_BASE},
    Duration, TransitionModeWrap,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSyncIdx() -> TransitionModeWrap {
    TransitionMode::SyncIdx.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSysTime(sys_time_ns: u64) -> TransitionModeWrap {
    TransitionMode::SysTime(
        DcSysTime::from_utc(ECAT_DC_SYS_TIME_BASE + std::time::Duration::from_nanos(sys_time_ns))
            .unwrap(),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeGPIO(gpio: GPIOIn) -> TransitionModeWrap {
    TransitionMode::GPIO(gpio).into()
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

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeNone() -> TransitionModeWrap {
    TransitionModeWrap {
        tag: autd3capi_driver::TransitionModeTag::None,
        value: 0,
    }
}
