use autd3_core::{ethercat::DcSysTime, geometry::Device};

#[repr(u8)]
#[derive(Clone, Copy, Default)]
pub enum GPIOOutputTypeTag {
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
pub union GPIOOutputTypeValue {
    pub null: u64,
    pub sys_time: DcSysTime,
    pub idx: u16,
    pub direct: bool,
}

#[repr(C)]
pub struct GPIOOutputTypeWrap {
    ty: GPIOOutputTypeTag,
    value: GPIOOutputTypeValue,
}

impl Default for GPIOOutputTypeWrap {
    fn default() -> Self {
        GPIOOutputTypeWrap {
            ty: GPIOOutputTypeTag::None,
            value: GPIOOutputTypeValue { null: 0 },
        }
    }
}

impl GPIOOutputTypeWrap {
    pub fn convert(self, dev: &Device) -> Option<autd3_driver::firmware::fpga::GPIOOutputType> {
        match self.ty {
            GPIOOutputTypeTag::None => None,
            GPIOOutputTypeTag::BaseSignal => {
                Some(autd3_driver::firmware::fpga::GPIOOutputType::BaseSignal)
            }
            GPIOOutputTypeTag::Thermo => Some(autd3_driver::firmware::fpga::GPIOOutputType::Thermo),
            GPIOOutputTypeTag::ForceFan => {
                Some(autd3_driver::firmware::fpga::GPIOOutputType::ForceFan)
            }
            GPIOOutputTypeTag::Sync => Some(autd3_driver::firmware::fpga::GPIOOutputType::Sync),
            GPIOOutputTypeTag::ModSegment => {
                Some(autd3_driver::firmware::fpga::GPIOOutputType::ModSegment)
            }
            GPIOOutputTypeTag::ModIdx => Some(
                autd3_driver::firmware::fpga::GPIOOutputType::ModIdx(unsafe { self.value.idx }),
            ),
            GPIOOutputTypeTag::StmSegment => {
                Some(autd3_driver::firmware::fpga::GPIOOutputType::StmSegment)
            }
            GPIOOutputTypeTag::StmIdx => Some(
                autd3_driver::firmware::fpga::GPIOOutputType::StmIdx(unsafe { self.value.idx }),
            ),
            GPIOOutputTypeTag::IsStmMode => {
                Some(autd3_driver::firmware::fpga::GPIOOutputType::IsStmMode)
            }
            GPIOOutputTypeTag::PwmOut => {
                Some(autd3_driver::firmware::fpga::GPIOOutputType::PwmOut(
                    &dev[unsafe { self.value.idx } as usize],
                ))
            }
            GPIOOutputTypeTag::Direct => Some(
                autd3_driver::firmware::fpga::GPIOOutputType::Direct(unsafe { self.value.direct }),
            ),
            GPIOOutputTypeTag::SysTimeEq => {
                Some(autd3_driver::firmware::fpga::GPIOOutputType::SysTimeEq(
                    unsafe { self.value.sys_time },
                ))
            }
        }
    }
}

impl From<Option<autd3_driver::firmware::fpga::GPIOOutputType<'_>>> for GPIOOutputTypeWrap {
    fn from(value: Option<autd3_driver::firmware::fpga::GPIOOutputType>) -> Self {
        match value {
            None => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::None,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::BaseSignal) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::BaseSignal,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::Thermo) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::Thermo,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::ForceFan) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::ForceFan,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::Sync) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::Sync,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::ModSegment) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::ModSegment,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::ModIdx(v)) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::ModIdx,
                value: GPIOOutputTypeValue { idx: v },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::StmSegment) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::StmSegment,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::StmIdx(v)) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::StmIdx,
                value: GPIOOutputTypeValue { idx: v },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::IsStmMode) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::IsStmMode,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::PwmOut(v)) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::PwmOut,
                value: GPIOOutputTypeValue { idx: v.idx() as _ },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::Direct(v)) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::Direct,
                value: GPIOOutputTypeValue { direct: v },
            },
            Some(autd3_driver::firmware::fpga::GPIOOutputType::SysTimeEq(v)) => {
                GPIOOutputTypeWrap {
                    ty: GPIOOutputTypeTag::SysTimeEq,
                    value: GPIOOutputTypeValue { sys_time: v },
                }
            }
            _ => unreachable!(),
        }
    }
}
