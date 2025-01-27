use autd3capi_driver::{
    autd3::{
        self,
        controller::{ParallelMode, Sleep, SpinSleeper},
    },
    ControllerPtr, DatagramPtr, Duration, DynDatagram, OptionDuration, ResultStatus, SenderPtr,
    SleeperWrap,
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

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSender(mut cnt: ControllerPtr, option: SenderOption) -> SenderPtr {
    SenderPtr(Box::into_raw(Box::new(cnt.sender(option.into()))) as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSenderSend(mut sender: SenderPtr, d: DatagramPtr) -> ResultStatus {
    sender.send(*Box::<DynDatagram>::from(d)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSpinSleepDefaultAccuracy() -> u32 {
    SpinSleeper::default().native_accuracy_ns()
}
