use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct LoopBehavior {
    pub(crate) rep: u32,
}

impl From<autd3_driver::common::LoopBehavior> for LoopBehavior {
    fn from(value: autd3_driver::common::LoopBehavior) -> Self {
        match value {
            autd3_driver::derive::LoopBehavior::Finite(v) => Self { rep: v.get() - 1 },
            autd3_driver::derive::LoopBehavior::Infinite => Self { rep: 0xFFFFFFFF },
        }
    }
}

impl From<LoopBehavior> for autd3_driver::common::LoopBehavior {
    fn from(value: LoopBehavior) -> Self {
        match value.rep {
            0xFFFFFFFF => autd3_driver::derive::LoopBehavior::Infinite,
            v => autd3_driver::derive::LoopBehavior::Finite(NonZeroU32::new(v + 1).unwrap()),
        }
    }
}

#[cfg(feature = "export")]
mod export {
    use super::*;

    #[no_mangle]
    #[must_use]
    pub unsafe extern "C" fn AUTDLoopBehaviorInfinite() -> LoopBehavior {
        autd3_driver::derive::LoopBehavior::Infinite.into()
    }

    #[no_mangle]
    #[must_use]
    pub unsafe extern "C" fn AUTDLoopBehaviorFinite(v: u32) -> LoopBehavior {
        autd3_driver::derive::LoopBehavior::Finite(NonZeroU32::new(v).unwrap()).into()
    }

    #[no_mangle]
    #[must_use]
    pub unsafe extern "C" fn AUTDLoopBehaviorOnce() -> LoopBehavior {
        autd3_driver::derive::LoopBehavior::once().into()
    }
}
