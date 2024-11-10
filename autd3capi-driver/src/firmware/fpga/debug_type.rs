use autd3::prelude::DcSysTime;
use autd3_driver::geometry::Device;

#[repr(u8)]
#[derive(Clone, Copy, Default)]
enum DebugTypeTag {
    #[default]
    None = 0,
    BaseSignal = 1,
    Thermo = 2,
    ForceFan = 3,
    Sync = 4,
    ModSegment = 5,
    ModIdx = 6,
    StmSegment = 7,
    StmIdx = 8,
    IsStmMode = 9,
    PwmOut = 10,
    Direct = 11,
    SysTimeEq = 12,
}

#[repr(C)]
pub union DebugTypeValue {
    pub null: u64,
    pub sys_time: DcSysTime,
    pub idx: u16,
    pub direct: bool,
}

#[repr(C)]
pub struct DebugTypeWrap {
    ty: DebugTypeTag,
    value: DebugTypeValue,
}

impl Default for DebugTypeWrap {
    fn default() -> Self {
        DebugTypeWrap {
            ty: DebugTypeTag::None,
            value: DebugTypeValue { null: 0 },
        }
    }
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
            DebugTypeTag::ModIdx => {
                autd3_driver::firmware::fpga::DebugType::ModIdx(unsafe { self.value.idx })
            }
            DebugTypeTag::StmSegment => autd3_driver::firmware::fpga::DebugType::StmSegment,
            DebugTypeTag::StmIdx => {
                autd3_driver::firmware::fpga::DebugType::StmIdx(unsafe { self.value.idx })
            }
            DebugTypeTag::IsStmMode => autd3_driver::firmware::fpga::DebugType::IsStmMode,
            DebugTypeTag::PwmOut => autd3_driver::firmware::fpga::DebugType::PwmOut(
                &dev[unsafe { self.value.idx } as usize],
            ),
            DebugTypeTag::Direct => {
                autd3_driver::firmware::fpga::DebugType::Direct(unsafe { self.value.direct })
            }
            DebugTypeTag::SysTimeEq => {
                autd3_driver::firmware::fpga::DebugType::SysTimeEq(unsafe { self.value.sys_time })
            }
        }
    }
}

impl<'a> From<autd3_driver::firmware::fpga::DebugType<'a>> for DebugTypeWrap {
    fn from(value: autd3_driver::firmware::fpga::DebugType) -> Self {
        match value {
            autd3_driver::firmware::fpga::DebugType::None => DebugTypeWrap {
                ty: DebugTypeTag::None,
                value: DebugTypeValue { null: 0 },
            },
            autd3_driver::firmware::fpga::DebugType::BaseSignal => DebugTypeWrap {
                ty: DebugTypeTag::BaseSignal,
                value: DebugTypeValue { null: 0 },
            },
            autd3_driver::firmware::fpga::DebugType::Thermo => DebugTypeWrap {
                ty: DebugTypeTag::Thermo,
                value: DebugTypeValue { null: 0 },
            },
            autd3_driver::firmware::fpga::DebugType::ForceFan => DebugTypeWrap {
                ty: DebugTypeTag::ForceFan,
                value: DebugTypeValue { null: 0 },
            },
            autd3_driver::firmware::fpga::DebugType::Sync => DebugTypeWrap {
                ty: DebugTypeTag::Sync,
                value: DebugTypeValue { null: 0 },
            },
            autd3_driver::firmware::fpga::DebugType::ModSegment => DebugTypeWrap {
                ty: DebugTypeTag::ModSegment,
                value: DebugTypeValue { null: 0 },
            },
            autd3_driver::firmware::fpga::DebugType::ModIdx(v) => DebugTypeWrap {
                ty: DebugTypeTag::ModIdx,
                value: DebugTypeValue { idx: v },
            },
            autd3_driver::firmware::fpga::DebugType::StmSegment => DebugTypeWrap {
                ty: DebugTypeTag::StmSegment,
                value: DebugTypeValue { null: 0 },
            },
            autd3_driver::firmware::fpga::DebugType::StmIdx(v) => DebugTypeWrap {
                ty: DebugTypeTag::StmIdx,
                value: DebugTypeValue { idx: v },
            },
            autd3_driver::firmware::fpga::DebugType::IsStmMode => DebugTypeWrap {
                ty: DebugTypeTag::IsStmMode,
                value: DebugTypeValue { null: 0 },
            },
            autd3_driver::firmware::fpga::DebugType::PwmOut(v) => DebugTypeWrap {
                ty: DebugTypeTag::PwmOut,
                value: DebugTypeValue { idx: v.idx() as _ },
            },
            autd3_driver::firmware::fpga::DebugType::Direct(v) => DebugTypeWrap {
                ty: DebugTypeTag::Direct,
                value: DebugTypeValue { direct: v },
            },
            autd3_driver::firmware::fpga::DebugType::SysTimeEq(v) => DebugTypeWrap {
                ty: DebugTypeTag::SysTimeEq,
                value: DebugTypeValue { sys_time: v },
            },
            _ => unreachable!(),
        }
    }
}
