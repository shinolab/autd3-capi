use autd3capi_driver::driver::{defined::rad, firmware::fpga::Phase};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseFromRad(value: f32) -> u8 {
    Phase::from(value * rad).0
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseToRad(value: Phase) -> f32 {
    value.radian()
}
