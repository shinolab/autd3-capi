use autd3::core::sleep::{Sleeper, SpinWaitSleeper, StdSleeper};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleeperTag {
    Std = 0,
    SpinWait = 4,
}

impl From<SleeperTag> for Box<dyn Sleeper> {
    fn from(tag: SleeperTag) -> Self {
        match tag {
            SleeperTag::Std => Box::new(StdSleeper),
            SleeperTag::SpinWait => Box::new(SpinWaitSleeper),
        }
    }
}
