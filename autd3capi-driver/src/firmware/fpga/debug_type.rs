use autd3::core::{ethercat::DcSysTime, geometry::Device};

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
    SyncDiff = 13,
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
    pub fn convert(self, dev: &Device) -> Option<autd3::driver::datagram::GPIOOutputType> {
        match self.ty {
            GPIOOutputTypeTag::None => None,
            GPIOOutputTypeTag::BaseSignal => {
                Some(autd3::driver::datagram::GPIOOutputType::BaseSignal)
            }
            GPIOOutputTypeTag::Thermo => Some(autd3::driver::datagram::GPIOOutputType::Thermo),
            GPIOOutputTypeTag::ForceFan => Some(autd3::driver::datagram::GPIOOutputType::ForceFan),
            GPIOOutputTypeTag::Sync => Some(autd3::driver::datagram::GPIOOutputType::Sync),
            GPIOOutputTypeTag::ModSegment => {
                Some(autd3::driver::datagram::GPIOOutputType::ModSegment)
            }
            GPIOOutputTypeTag::ModIdx => {
                Some(autd3::driver::datagram::GPIOOutputType::ModIdx(unsafe {
                    self.value.idx
                }))
            }
            GPIOOutputTypeTag::StmSegment => {
                Some(autd3::driver::datagram::GPIOOutputType::StmSegment)
            }
            GPIOOutputTypeTag::StmIdx => {
                Some(autd3::driver::datagram::GPIOOutputType::StmIdx(unsafe {
                    self.value.idx
                }))
            }
            GPIOOutputTypeTag::IsStmMode => {
                Some(autd3::driver::datagram::GPIOOutputType::IsStmMode)
            }
            GPIOOutputTypeTag::PwmOut => Some(autd3::driver::datagram::GPIOOutputType::PwmOut(
                &dev[unsafe { self.value.idx } as usize],
            )),
            GPIOOutputTypeTag::Direct => {
                Some(autd3::driver::datagram::GPIOOutputType::Direct(unsafe {
                    self.value.direct
                }))
            }
            GPIOOutputTypeTag::SysTimeEq => {
                Some(autd3::driver::datagram::GPIOOutputType::SysTimeEq(unsafe {
                    self.value.sys_time
                }))
            }
            GPIOOutputTypeTag::SyncDiff => Some(autd3::driver::datagram::GPIOOutputType::SyncDiff),
        }
    }
}

impl From<Option<autd3::driver::datagram::GPIOOutputType<'_>>> for GPIOOutputTypeWrap {
    fn from(value: Option<autd3::driver::datagram::GPIOOutputType>) -> Self {
        match value {
            None => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::None,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3::driver::datagram::GPIOOutputType::BaseSignal) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::BaseSignal,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3::driver::datagram::GPIOOutputType::Thermo) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::Thermo,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3::driver::datagram::GPIOOutputType::ForceFan) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::ForceFan,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3::driver::datagram::GPIOOutputType::Sync) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::Sync,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3::driver::datagram::GPIOOutputType::ModSegment) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::ModSegment,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3::driver::datagram::GPIOOutputType::ModIdx(v)) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::ModIdx,
                value: GPIOOutputTypeValue { idx: v },
            },
            Some(autd3::driver::datagram::GPIOOutputType::StmSegment) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::StmSegment,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3::driver::datagram::GPIOOutputType::StmIdx(v)) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::StmIdx,
                value: GPIOOutputTypeValue { idx: v },
            },
            Some(autd3::driver::datagram::GPIOOutputType::IsStmMode) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::IsStmMode,
                value: GPIOOutputTypeValue { null: 0 },
            },
            Some(autd3::driver::datagram::GPIOOutputType::PwmOut(v)) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::PwmOut,
                value: GPIOOutputTypeValue { idx: v.idx() as _ },
            },
            Some(autd3::driver::datagram::GPIOOutputType::Direct(v)) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::Direct,
                value: GPIOOutputTypeValue { direct: v },
            },
            Some(autd3::driver::datagram::GPIOOutputType::SysTimeEq(v)) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::SysTimeEq,
                value: GPIOOutputTypeValue { sys_time: v },
            },
            Some(autd3::driver::datagram::GPIOOutputType::SyncDiff) => GPIOOutputTypeWrap {
                ty: GPIOOutputTypeTag::SyncDiff,
                value: GPIOOutputTypeValue { null: 0 },
            },
            _ => unreachable!(),
        }
    }
}
