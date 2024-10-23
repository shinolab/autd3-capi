use autd3::prelude::AsyncSleeper;
use spin_sleep::{SpinSleeper, SpinStrategy};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum TimerStrategyTag {
    Std = 0,
    Spin = 1,
    Async = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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
pub struct TimerStrategyWrap {
    pub tag: TimerStrategyTag,
    pub value: u32,
    pub spin_strategy: SpinStrategyTag,
}

impl From<TimerStrategyWrap> for autd3::controller::TimerStrategy {
    fn from(value: TimerStrategyWrap) -> Self {
        match value.tag {
            TimerStrategyTag::Std => autd3::controller::TimerStrategy::Std,
            TimerStrategyTag::Spin => autd3::controller::TimerStrategy::Spin(
                SpinSleeper::new(value.value).with_spin_strategy(value.spin_strategy.into()),
            ),
            TimerStrategyTag::Async => {
                autd3::controller::TimerStrategy::Async(AsyncSleeper::default())
                // todo
            }
        }
    }
}
