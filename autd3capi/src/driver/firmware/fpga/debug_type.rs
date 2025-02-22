use autd3capi_driver::{
    DebugTypeWrap, TransducerPtr, autd3::driver::firmware::fpga::DebugType,
    driver::ethercat::DcSysTime,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeNone() -> DebugTypeWrap {
    DebugType::None.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeBaseSignal() -> DebugTypeWrap {
    DebugType::BaseSignal.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeThermo() -> DebugTypeWrap {
    DebugType::Thermo.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeForceFan() -> DebugTypeWrap {
    DebugType::ForceFan.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeSync() -> DebugTypeWrap {
    DebugType::Sync.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeModSegment() -> DebugTypeWrap {
    DebugType::ModSegment.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeModIdx(value: u16) -> DebugTypeWrap {
    DebugType::ModIdx(value).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeStmSegment() -> DebugTypeWrap {
    DebugType::StmSegment.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeStmIdx(value: u16) -> DebugTypeWrap {
    DebugType::StmIdx(value).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeIsStmMode() -> DebugTypeWrap {
    DebugType::IsStmMode.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypePwmOut(value: TransducerPtr) -> DebugTypeWrap {
    DebugType::PwmOut(&value).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeDirect(value: bool) -> DebugTypeWrap {
    DebugType::Direct(value).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeSysTimeEq(sys_time: DcSysTime) -> DebugTypeWrap {
    DebugType::SysTimeEq(sys_time).into()
}
