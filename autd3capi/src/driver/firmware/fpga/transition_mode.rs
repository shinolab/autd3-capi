use autd3capi_driver::{
    TransitionModeWrap, autd3::core::datagram::GPIOIn, driver::ethercat::DcSysTime,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSyncIdx() -> TransitionModeWrap {
    TransitionModeWrap {
        tag: autd3capi_driver::TransitionModeTag::SyncIdx,
        value: autd3capi_driver::TransitionModeValue { null: 0 },
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeSysTime(sys_time: DcSysTime) -> TransitionModeWrap {
    TransitionModeWrap {
        tag: autd3capi_driver::TransitionModeTag::SysTime,
        value: autd3capi_driver::TransitionModeValue { sys_time },
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeGPIO(gpio: GPIOIn) -> TransitionModeWrap {
    TransitionModeWrap {
        tag: autd3capi_driver::TransitionModeTag::Gpio,
        value: autd3capi_driver::TransitionModeValue { gpio_in: gpio },
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeExt() -> TransitionModeWrap {
    TransitionModeWrap {
        tag: autd3capi_driver::TransitionModeTag::Ext,
        value: autd3capi_driver::TransitionModeValue { null: 0 },
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeImmediate() -> TransitionModeWrap {
    TransitionModeWrap {
        tag: autd3capi_driver::TransitionModeTag::Immediate,
        value: autd3capi_driver::TransitionModeValue { null: 0 },
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDTransitionModeLater() -> TransitionModeWrap {
    TransitionModeWrap {
        tag: autd3capi_driver::TransitionModeTag::Later,
        value: autd3capi_driver::TransitionModeValue { null: 0 },
    }
}
