use autd3capi_driver::{
    TransitionModeWrap,
    autd3::core::{datagram::GPIOIn, derive::TransitionMode},
    driver::ethercat::DcSysTime,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSyncIdx() -> TransitionModeWrap {
    TransitionMode::SyncIdx.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSysTime(sys_time: DcSysTime) -> TransitionModeWrap {
    TransitionMode::SysTime(sys_time).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeGPIO(gpio: GPIOIn) -> TransitionModeWrap {
    TransitionMode::GPIO(gpio).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeExt() -> TransitionModeWrap {
    TransitionMode::Ext.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeImmediate() -> TransitionModeWrap {
    TransitionMode::Immediate.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeNone() -> TransitionModeWrap {
    TransitionModeWrap {
        tag: autd3capi_driver::TransitionModeTag::None,
        value: autd3capi_driver::TransitionModeValue { null: 0 },
    }
}
