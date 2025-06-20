use autd3capi_driver::{
    ControllerPtr, DatagramPtr, Duration, OptionDuration, ResultStatus, SenderPtr,
    TimerStrategyWrap,
    autd3::{self, controller::ParallelMode},
    core::sleep::{Sleep, SpinSleeper},
    driver::firmware::driver::{BoxedDatagram, TimerStrategy},
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SenderOption {
    pub send_interval: Duration,
    pub receive_interval: Duration,
    pub timeout: OptionDuration,
    pub parallel: ParallelMode,
    pub strict: bool,
}

impl From<SenderOption> for autd3::controller::SenderOption {
    fn from(value: SenderOption) -> Self {
        autd3::controller::SenderOption {
            send_interval: value.send_interval.into(),
            receive_interval: value.receive_interval.into(),
            timeout: value.timeout.into(),
            parallel: value.parallel,
            strict: value.strict,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDSetDefaultSenderOption(mut cnt: ControllerPtr, option: SenderOption) {
    cnt.default_sender_option = option.into();
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSender(
    mut cnt: ControllerPtr,
    option: SenderOption,
    timer_strategy: TimerStrategyWrap,
) -> SenderPtr {
    SenderPtr(Box::into_raw(Box::new(cnt.sender(
        option.into(),
        Box::<dyn TimerStrategy<Box<dyn Sleep>>>::from(timer_strategy),
    ))) as *const _ as _)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSenderSend(mut sender: SenderPtr, d: DatagramPtr) -> ResultStatus {
    sender.send(*Box::<BoxedDatagram>::from(d)).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSpinSleepDefaultAccuracy() -> u32 {
    SpinSleeper::default().native_accuracy_ns()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSenderOptionIsDefault(option: SenderOption) -> bool {
    autd3::controller::SenderOption::from(option) == Default::default()
}
