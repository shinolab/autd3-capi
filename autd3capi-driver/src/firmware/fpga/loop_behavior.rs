use std::num::NonZeroU16;

#[repr(C)]
pub struct LoopBehavior {
    pub rep: u16,
}

impl From<autd3::driver::firmware::fpga::LoopBehavior> for LoopBehavior {
    fn from(value: autd3::driver::firmware::fpga::LoopBehavior) -> Self {
        LoopBehavior { rep: value.rep() }
    }
}

impl From<LoopBehavior> for autd3::driver::firmware::fpga::LoopBehavior {
    fn from(value: LoopBehavior) -> Self {
        match value.rep {
            0xFFFF => autd3::driver::firmware::fpga::LoopBehavior::Infinite,
            v => {
                autd3::driver::firmware::fpga::LoopBehavior::Finite(NonZeroU16::new(v + 1).unwrap())
            }
        }
    }
}
