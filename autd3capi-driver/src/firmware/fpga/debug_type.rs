use autd3::prelude::DcSysTime;
use autd3_driver::{ethercat::ECAT_DC_SYS_TIME_BASE, geometry::Device};

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
#[derive(Clone, Copy, Default)]
pub struct DebugTypeWrap {
    ty: DebugTypeTag,
    value: u64,
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
                autd3_driver::firmware::fpga::DebugType::ModIdx(self.value as _)
            }
            DebugTypeTag::StmSegment => autd3_driver::firmware::fpga::DebugType::StmSegment,
            DebugTypeTag::StmIdx => {
                autd3_driver::firmware::fpga::DebugType::StmIdx(self.value as _)
            }
            DebugTypeTag::IsStmMode => autd3_driver::firmware::fpga::DebugType::IsStmMode,
            DebugTypeTag::PwmOut => {
                autd3_driver::firmware::fpga::DebugType::PwmOut(&dev[self.value as usize])
            }
            DebugTypeTag::Direct => {
                autd3_driver::firmware::fpga::DebugType::Direct(self.value != 0)
            }
            DebugTypeTag::SysTimeEq => autd3_driver::firmware::fpga::DebugType::SysTimeEq(
                DcSysTime::from_utc(
                    ECAT_DC_SYS_TIME_BASE + std::time::Duration::from_nanos(self.value),
                )
                .unwrap(),
            ),
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
                value: v as _,
            },
            autd3_driver::firmware::fpga::DebugType::StmSegment => DebugTypeWrap {
                ty: DebugTypeTag::StmSegment,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::StmIdx(v) => DebugTypeWrap {
                ty: DebugTypeTag::StmIdx,
                value: v as _,
            },
            autd3_driver::firmware::fpga::DebugType::IsStmMode => DebugTypeWrap {
                ty: DebugTypeTag::IsStmMode,
                value: 0,
            },
            autd3_driver::firmware::fpga::DebugType::PwmOut(v) => DebugTypeWrap {
                ty: DebugTypeTag::PwmOut,
                value: v.idx() as _,
            },
            autd3_driver::firmware::fpga::DebugType::Direct(v) => DebugTypeWrap {
                ty: DebugTypeTag::Direct,
                value: if v { 1 } else { 0 },
            },
            autd3_driver::firmware::fpga::DebugType::SysTimeEq(v) => DebugTypeWrap {
                ty: DebugTypeTag::SysTimeEq,
                value: v.sys_time(),
            },
            _ => unreachable!(),
        }
    }
}
