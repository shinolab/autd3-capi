use autd3capi_driver::{
    GPIOOutputTypeWrap, TransducerPtr, driver::datagram::GPIOOutputType,
    driver::ethercat::DcSysTime,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeNone() -> GPIOOutputTypeWrap {
    None.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeBaseSignal() -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::BaseSignal).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeThermo() -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::Thermo).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeForceFan() -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::ForceFan).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeSync() -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::Sync).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeModSegment() -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::ModSegment).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeModIdx(value: u16) -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::ModIdx(value)).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeStmSegment() -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::StmSegment).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeStmIdx(value: u16) -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::StmIdx(value)).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeIsStmMode() -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::IsStmMode).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypePwmOut(value: TransducerPtr) -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::PwmOut(&value)).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeDirect(value: bool) -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::Direct(value)).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeSysTimeEq(sys_time: DcSysTime) -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::SysTimeEq(sys_time)).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeSyncDiff() -> GPIOOutputTypeWrap {
    Some(GPIOOutputType::SyncDiff).into()
}
