use autd3_driver::common::Phase;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseFromRad(value: f64) -> u8 {
    Phase::from_rad(value).value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseToRad(value: u8) -> f64 {
    Phase::new(value).radian()
}
