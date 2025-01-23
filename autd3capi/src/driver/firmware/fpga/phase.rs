use autd3capi_driver::{autd3::prelude::rad, driver::firmware::fpga::Phase};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseFromRad(value: f32) -> u8 {
    Phase::from(value * rad).0
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseToRad(value: u8) -> f32 {
    Phase(value).radian()
}
