use autd3capi_driver::{
    autd3::{
        self,
        controller::{ParallelMode, Sleep},
    },
    Duration, OptionDuration, SleeperWrap,
};

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
