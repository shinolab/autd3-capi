use autd3::derive::Device;

#[repr(u8)]
#[derive(Clone, Copy, Default)]
enum DebugTypeTag {
    #[default]
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
#[derive(Clone, Copy, Default)]
pub struct DebugTypeWrap {
    ty: DebugTypeTag,
    value: u16,
}

impl DebugTypeWrap {
    pub fn convert(self, dev: &Device) -> autd3_driver::firmware::fpga::DebugType {
        match self.ty {
            DebugTypeTag::None => autd3_driver::firmware::fpga::DebugType::None,
            DebugTypeTag::BaseSignal => autd3_driver::firmware::fpga::DebugType::BaseSignal,
            DebugTypeTag::Thermo => autd3_driver::firmware::fpga::DebugType::Thermo,
            DebugTypeTag::ForceFan => autd3_driver::firmware::fpga::DebugType::ForceFan,
            DebugTypeTag::Sync => autd3_driver::firmware::fpga::DebugType::Sync,
            DebugTypeTag::ModSegment => autd3_driver::firmware::fpga::DebugType::ModSegment,
            DebugTypeTag::ModIdx => autd3_driver::firmware::fpga::DebugType::ModIdx(self.value),
            DebugTypeTag::StmSegment => autd3_driver::firmware::fpga::DebugType::StmSegment,
            DebugTypeTag::StmIdx => autd3_driver::firmware::fpga::DebugType::StmIdx(self.value),
            DebugTypeTag::IsStmMode => autd3_driver::firmware::fpga::DebugType::IsStmMode,
            DebugTypeTag::PwmOut => {
                autd3_driver::firmware::fpga::DebugType::PwmOut(&dev[self.value as usize])
            }
            DebugTypeTag::Direct => {
                autd3_driver::firmware::fpga::DebugType::Direct(self.value != 0)
            }
        }
    }
}

impl<'a> From<autd3_driver::firmware::fpga::DebugType<'a>> for DebugTypeWrap {
    fn from(value: autd3_driver::firmware::fpga::DebugType) -> Self {
        match value {
            autd3_driver::firmware::fpga::DebugType::None => DebugTypeWrap {
                ty: DebugTypeTag::None,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::BaseSignal => DebugTypeWrap {
                ty: DebugTypeTag::BaseSignal,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::Thermo => DebugTypeWrap {
                ty: DebugTypeTag::Thermo,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::ForceFan => DebugTypeWrap {
                ty: DebugTypeTag::ForceFan,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::Sync => DebugTypeWrap {
                ty: DebugTypeTag::Sync,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::ModSegment => DebugTypeWrap {
                ty: DebugTypeTag::ModSegment,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::ModIdx(v) => DebugTypeWrap {
                ty: DebugTypeTag::ModIdx,
                value: v,
            },
            autd3_driver::firmware::fpga::DebugType::StmSegment => DebugTypeWrap {
                ty: DebugTypeTag::StmSegment,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::StmIdx(v) => DebugTypeWrap {
                ty: DebugTypeTag::StmIdx,
                value: v,
            },
            autd3_driver::firmware::fpga::DebugType::IsStmMode => DebugTypeWrap {
                ty: DebugTypeTag::IsStmMode,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::PwmOut(v) => DebugTypeWrap {
                ty: DebugTypeTag::PwmOut,
                value: v.idx() as u16,
            },
            autd3_driver::firmware::fpga::DebugType::Direct(v) => DebugTypeWrap {
                ty: DebugTypeTag::Direct,
                value: if v { 1 } else { 0 },
            },
            _ => unreachable!(),
        }
    }
}
