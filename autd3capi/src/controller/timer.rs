use autd3capi_driver::{
    autd3::controller::timer::SpinSleeper, SpinStrategyTag, TimerStrategyTag, TimerStrategyWrap,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTimerStrategyStd() -> TimerStrategyWrap {
    TimerStrategyWrap {
        tag: TimerStrategyTag::Std,
        value: 0,
        spin_strategy: SpinStrategyTag::SpinLoopHint,
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTimerStrategySpinDefault() -> TimerStrategyWrap {
    TimerStrategyWrap {
        tag: TimerStrategyTag::Spin,
        value: SpinSleeper::default().native_accuracy_ns(),
        spin_strategy: SpinSleeper::default().spin_strategy().into(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTimerStrategySpin(
    native_accuracy_ns: u32,
    spin_strategy: SpinStrategyTag,
) -> TimerStrategyWrap {
    TimerStrategyWrap {
        tag: TimerStrategyTag::Spin,
        value: native_accuracy_ns,
        spin_strategy,
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTimerStrategyAsync(timer_resolution: u32) -> TimerStrategyWrap {
    TimerStrategyWrap {
        tag: TimerStrategyTag::Async,
        value: timer_resolution,
        spin_strategy: SpinStrategyTag::SpinLoopHint,
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTimerStrategyWaitable() -> TimerStrategyWrap {
    TimerStrategyWrap {
        tag: TimerStrategyTag::Waitable,
        value: 0,
        spin_strategy: SpinStrategyTag::SpinLoopHint,
    }
}
