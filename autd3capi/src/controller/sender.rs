use autd3capi_driver::{
    ControllerPtr, DatagramPtr, Duration, OptionDuration, ResultStatus, SenderPtr, SleeperWrap,
    autd3::{
        self,
        controller::{ParallelMode, Sleep, SpinSleeper},
    },
    driver::datagram::BoxedDatagram,
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SenderOption {
    pub send_interval: Duration,
    pub receive_interval: Duration,
    pub timeout: OptionDuration,
    pub parallel: ParallelMode,
    pub sleeper: SleeperWrap,
}

impl From<SenderOption> for autd3::controller::SenderOption<Box<dyn Sleep>> {
    fn from(value: SenderOption) -> Self {
        autd3::controller::SenderOption {
            send_interval: value.send_interval.into(),
            receive_interval: value.receive_interval.into(),
            timeout: value.timeout.into(),
            parallel: value.parallel,
            sleeper: value.sleeper.into(),
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSender(mut cnt: ControllerPtr, option: SenderOption) -> SenderPtr {
    SenderPtr(Box::into_raw(Box::new(cnt.sender(option.into()))) as *const _ as _)
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
    let autd3::controller::SenderOption::<SpinSleeper> {
        send_interval: default_send_interval,
        receive_interval: default_receive_interval,
        timeout: default_timeout,
        parallel: default_parallel,
        sleeper: _,
    } = autd3::controller::SenderOption::default();
    let autd3::controller::SenderOption::<Box<dyn Sleep>> {
        send_interval,
        receive_interval,
        timeout,
        parallel,
        sleeper: _,
    } = option.into();
    default_send_interval == send_interval
        && default_receive_interval == receive_interval
        && default_timeout == timeout
        && default_parallel == parallel
        && option.sleeper.tag == autd3capi_driver::SleeperTag::Spin
        && option.sleeper.value == SpinSleeper::default().native_accuracy_ns()
        && option.sleeper.spin_strategy
            == autd3capi_driver::SpinStrategyTag::from(SpinSleeper::default().spin_strategy())
}
