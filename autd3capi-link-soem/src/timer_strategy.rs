#[repr(u8)]
pub enum TimerStrategy {
    Sleep = 0,
    BusyWait = 1,
    NativeTimer = 2,
}

impl From<TimerStrategy> for autd3_link_soem::TimerStrategy {
    fn from(strategy: TimerStrategy) -> Self {
        match strategy {
            TimerStrategy::Sleep => autd3_link_soem::TimerStrategy::Sleep,
            TimerStrategy::NativeTimer => autd3_link_soem::TimerStrategy::NativeTimer,
            TimerStrategy::BusyWait => autd3_link_soem::TimerStrategy::BusyWait,
        }
    }
}
