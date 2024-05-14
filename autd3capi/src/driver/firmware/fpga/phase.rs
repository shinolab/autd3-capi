use autd3capi_driver::{autd3::derive::rad, driver::firmware::fpga::Phase};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseFromRad(value: f64) -> u8 {
    Phase::from(value * rad).value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseToRad(value: u8) -> f64 {
    Phase::new(value).radian()
}
