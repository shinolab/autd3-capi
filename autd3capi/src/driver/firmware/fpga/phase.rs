use autd3capi_driver::driver::{common::rad, firmware::fpga::Phase};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseFromRad(value: f32) -> u8 {
    Phase::from(value * rad).0
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseToRad(value: Phase) -> f32 {
    value.radian()
}
