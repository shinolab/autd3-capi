use std::num::NonZeroU16;

#[repr(C)]
pub struct LoopBehavior {
    pub rep: u16,
}

impl From<autd3::core::datagram::LoopBehavior> for LoopBehavior {
    fn from(value: autd3::core::datagram::LoopBehavior) -> Self {
        LoopBehavior { rep: value.rep() }
    }
}

impl From<LoopBehavior> for autd3::core::datagram::LoopBehavior {
    fn from(value: LoopBehavior) -> Self {
        match value.rep {
            0xFFFF => autd3::core::datagram::LoopBehavior::Infinite,
            v => autd3::core::datagram::LoopBehavior::Finite(NonZeroU16::new(v + 1).unwrap()),
        }
    }
}
