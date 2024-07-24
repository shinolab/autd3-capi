#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct LoopBehavior {
    pub(crate) rep: u16,
}

impl From<autd3_driver::firmware::fpga::LoopBehavior> for LoopBehavior {
    fn from(value: autd3_driver::firmware::fpga::LoopBehavior) -> Self {
        Self { rep: value.rep() }
    }
}

impl From<LoopBehavior> for autd3_driver::firmware::fpga::LoopBehavior {
    fn from(value: LoopBehavior) -> Self {
        match value.rep {
            0xFFFF => autd3_driver::firmware::fpga::LoopBehavior::infinite(),
            v => autd3_driver::firmware::fpga::LoopBehavior::finite(v + 1).unwrap(),
        }
    }
}
