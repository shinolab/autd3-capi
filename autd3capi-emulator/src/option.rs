use autd3capi_driver::Duration;

#[repr(C)]
pub struct RmsRecordOption {
    pub sound_speed: f32,
    pub time_step: Duration,
    pub print_progress: bool,
    pub memory_limits_hint_mb: u64,
    pub gpu: bool,
}

impl From<RmsRecordOption> for autd3_emulator::RmsRecordOption {
    fn from(value: RmsRecordOption) -> Self {
        autd3_emulator::RmsRecordOption {
            sound_speed: value.sound_speed,
            print_progress: value.print_progress,
            gpu: value.gpu,
        }
    }
}
