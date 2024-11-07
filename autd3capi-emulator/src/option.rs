use autd3capi_driver::Duration;

#[repr(C)]
pub struct RecordOption {
    pub sound_speed: f32,
    pub time_step: Duration,
    pub print_progress: bool,
    pub memory_limits_hint_mb: u64,
    pub gpu: bool,
}

impl From<RecordOption> for autd3_emulator::RecordOption {
    fn from(value: RecordOption) -> Self {
        autd3_emulator::RecordOption {
            sound_speed: value.sound_speed,
            time_step: value.time_step.into(),
            print_progress: value.print_progress,
            memory_limits_hint_mb: value.memory_limits_hint_mb as _,
            gpu: value.gpu,
        }
    }
}
