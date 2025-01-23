use autd3capi_driver::{
    autd3::driver::firmware::fpga::DebugType, driver::ethercat::DcSysTime, DebugTypeWrap,
    TransducerPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeNone() -> DebugTypeWrap {
    DebugType::None.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeBaseSignal() -> DebugTypeWrap {
    DebugType::BaseSignal.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeThermo() -> DebugTypeWrap {
    DebugType::Thermo.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeForceFan() -> DebugTypeWrap {
    DebugType::ForceFan.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeSync() -> DebugTypeWrap {
    DebugType::Sync.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeModSegment() -> DebugTypeWrap {
    DebugType::ModSegment.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeModIdx(value: u16) -> DebugTypeWrap {
    DebugType::ModIdx(value).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeStmSegment() -> DebugTypeWrap {
    DebugType::StmSegment.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeStmIdx(value: u16) -> DebugTypeWrap {
    DebugType::StmIdx(value).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeIsStmMode() -> DebugTypeWrap {
    DebugType::IsStmMode.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypePwmOut(value: TransducerPtr) -> DebugTypeWrap {
    DebugType::PwmOut(&value).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeDirect(value: bool) -> DebugTypeWrap {
    DebugType::Direct(value).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDebugTypeSysTimeEq(sys_time: DcSysTime) -> DebugTypeWrap {
    DebugType::SysTimeEq(sys_time).into()
}
