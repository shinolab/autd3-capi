#[repr(u8)]
pub enum TimerStrategy {
    Sleep = 0,
    BusyWait = 1,
}

impl From<TimerStrategy> for autd3_link_soem::TimerStrategy {
    fn from(strategy: TimerStrategy) -> Self {
        match strategy {
            TimerStrategy::Sleep => autd3_link_soem::TimerStrategy::Sleep,
            TimerStrategy::BusyWait => autd3_link_soem::TimerStrategy::BusyWait,
        }
    }
}
