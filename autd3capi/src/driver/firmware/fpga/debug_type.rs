use autd3capi_driver::{
    GPIOOutputTypeWrap, TransducerPtr, autd3::driver::firmware::fpga::GPIOOutputType,
    driver::ethercat::DcSysTime,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeNone() -> GPIOOutputTypeWrap {
    GPIOOutputType::None.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeBaseSignal() -> GPIOOutputTypeWrap {
    GPIOOutputType::BaseSignal.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeThermo() -> GPIOOutputTypeWrap {
    GPIOOutputType::Thermo.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeForceFan() -> GPIOOutputTypeWrap {
    GPIOOutputType::ForceFan.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeSync() -> GPIOOutputTypeWrap {
    GPIOOutputType::Sync.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeModSegment() -> GPIOOutputTypeWrap {
    GPIOOutputType::ModSegment.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeModIdx(value: u16) -> GPIOOutputTypeWrap {
    GPIOOutputType::ModIdx(value).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeStmSegment() -> GPIOOutputTypeWrap {
    GPIOOutputType::StmSegment.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeStmIdx(value: u16) -> GPIOOutputTypeWrap {
    GPIOOutputType::StmIdx(value).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeIsStmMode() -> GPIOOutputTypeWrap {
    GPIOOutputType::IsStmMode.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypePwmOut(value: TransducerPtr) -> GPIOOutputTypeWrap {
    GPIOOutputType::PwmOut(&value).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeDirect(value: bool) -> GPIOOutputTypeWrap {
    GPIOOutputType::Direct(value).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGPIOOutputTypeSysTimeEq(sys_time: DcSysTime) -> GPIOOutputTypeWrap {
    GPIOOutputType::SysTimeEq(sys_time).into()
}
