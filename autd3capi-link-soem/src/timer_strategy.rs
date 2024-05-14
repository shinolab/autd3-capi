#[repr(u8)]
pub enum TimerStrategy {
    Sleep = 0,
    BusyWait = 1,
    NativeTimer = 2,
}

impl From<TimerStrategy> for autd3::prelude::TimerStrategy {
    fn from(strategy: TimerStrategy) -> Self {
        match strategy {
            TimerStrategy::Sleep => autd3::prelude::TimerStrategy::Sleep,
            TimerStrategy::NativeTimer => autd3::prelude::TimerStrategy::NativeTimer,
            TimerStrategy::BusyWait => autd3::prelude::TimerStrategy::BusyWait,
        }
    }
}
