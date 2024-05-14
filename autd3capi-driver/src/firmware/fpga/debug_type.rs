use autd3::derive::Device;

#[repr(u8)]
pub enum DebugType {
    None,
    BaseSignal,
    Thermo,
    ForceFan,
    Sync,
    ModSegment,
    ModIdx,
    StmSegment,
    StmIdx,
    IsStmMode,
    PwmOut,
    Direct,
}

#[repr(C)]
pub struct DebugSetting {
    pub ty: DebugType,
    pub value: u16,
}

impl DebugSetting {
    pub fn convert(self, dev: &Device) -> autd3_driver::firmware::fpga::DebugType {
        match self.ty {
            DebugType::None => autd3_driver::firmware::fpga::DebugType::None,
            DebugType::BaseSignal => autd3_driver::firmware::fpga::DebugType::BaseSignal,
            DebugType::Thermo => autd3_driver::firmware::fpga::DebugType::Thermo,
            DebugType::ForceFan => autd3_driver::firmware::fpga::DebugType::ForceFan,
            DebugType::Sync => autd3_driver::firmware::fpga::DebugType::Sync,
            DebugType::ModSegment => autd3_driver::firmware::fpga::DebugType::ModSegment,
            DebugType::ModIdx => autd3_driver::firmware::fpga::DebugType::ModIdx(self.value),
            DebugType::StmSegment => autd3_driver::firmware::fpga::DebugType::StmSegment,
            DebugType::StmIdx => autd3_driver::firmware::fpga::DebugType::StmIdx(self.value),
            DebugType::IsStmMode => autd3_driver::firmware::fpga::DebugType::IsStmMode,
            DebugType::PwmOut => {
                autd3_driver::firmware::fpga::DebugType::PwmOut(&dev[self.value as usize])
            }
            DebugType::Direct => autd3_driver::firmware::fpga::DebugType::Direct(self.value != 0),
        }
    }
}
