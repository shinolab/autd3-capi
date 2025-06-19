use autd3::{
    controller::TimerStrategy,
    core::sleep::{Sleep, SpinSleeper, SpinStrategy, SpinWaitSleeper, StdSleeper},
    driver::firmware::driver::{FixedDelay, FixedSchedule},
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleeperTag {
    Std = 0,
    Spin = 1,
    SpinWait = 4,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinStrategyTag {
    YieldThread = 0,
    SpinLoopHint = 1,
}

impl From<SpinStrategyTag> for SpinStrategy {
    fn from(value: SpinStrategyTag) -> Self {
        match value {
            SpinStrategyTag::YieldThread => SpinStrategy::YieldThread,
            SpinStrategyTag::SpinLoopHint => SpinStrategy::SpinLoopHint,
        }
    }
}
impl From<SpinStrategy> for SpinStrategyTag {
    fn from(value: SpinStrategy) -> Self {
        match value {
            SpinStrategy::YieldThread => SpinStrategyTag::YieldThread,
            SpinStrategy::SpinLoopHint => SpinStrategyTag::SpinLoopHint,
            _ => unimplemented!(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SleeperWrap {
    pub tag: SleeperTag,
    pub value: u32,
    pub spin_strategy: SpinStrategyTag,
}

impl From<SleeperWrap> for Box<dyn Sleep> {
    fn from(value: SleeperWrap) -> Self {
        match value.tag {
            SleeperTag::Std => Box::new(StdSleeper),
            SleeperTag::Spin => Box::new(
                SpinSleeper::new(value.value).with_spin_strategy(value.spin_strategy.into()),
            ),
            SleeperTag::SpinWait => Box::new(SpinWaitSleeper),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerStrategyTag {
    FixedSchedule = 0,
    FixedDelay = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimerStrategyWrap {
    pub tag: TimerStrategyTag,
    pub sleep: SleeperWrap,
}

impl From<TimerStrategyWrap> for Box<dyn TimerStrategy<Box<dyn Sleep>>> {
    fn from(value: TimerStrategyWrap) -> Self {
        let sleep = Box::<dyn Sleep>::from(value.sleep);
        match value.tag {
            TimerStrategyTag::FixedSchedule => Box::new(FixedSchedule(sleep)),
            TimerStrategyTag::FixedDelay => Box::new(FixedDelay(sleep)),
        }
    }
}
