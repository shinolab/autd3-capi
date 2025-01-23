use autd3capi_driver::{
    autd3::core::{datagram::GPIOIn, derive::TransitionMode},
    driver::ethercat::DcSysTime,
    TransitionModeWrap,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSyncIdx() -> TransitionModeWrap {
    TransitionMode::SyncIdx.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSysTime(sys_time: DcSysTime) -> TransitionModeWrap {
    TransitionMode::SysTime(sys_time).into()
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
        value: autd3capi_driver::TransitionModeValue { null: 0 },
    }
}
