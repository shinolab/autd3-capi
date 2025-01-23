use std::num::NonZeroU32;

use autd3::controller::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SleeperTag {
    Std = 0,
    Spin = 1,
    Waitable = 3,
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
pub struct SleeperWrap {
    pub tag: SleeperTag,
    pub value: u32,
    pub spin_strategy: SpinStrategyTag,
}

impl From<SleeperWrap> for Box<dyn Sleep> {
    fn from(value: SleeperWrap) -> Self {
        match value.tag {
            SleeperTag::Std => Box::new(StdSleeper {
                timer_resolution: NonZeroU32::new(value.value),
            }),
            SleeperTag::Spin => Box::new(
                SpinSleeper::new(value.value).with_spin_strategy(value.spin_strategy.into()),
            ),
            #[cfg(target_os = "windows")]
            SleeperTag::Waitable => Box::new(
                autd3::controller::WaitableSleeper::new()
                    .expect("Failed to create WaitableSleeper"),
            ),
            #[cfg(not(target_os = "windows"))]
            TimerStrategyTag::Waitable => unimplemented!(),
        }
    }
}
