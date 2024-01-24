use autd3capi_def::{autd3::gain::Focus, driver::geometry::Vector3, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocus(x: float, y: float, z: float, intensity: u8) -> GainPtr {
    Focus::new(Vector3::new(x, y, z))
        .with_intensity(intensity)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocusDefaultIntensity() -> u8 {
    Focus::new(Vector3::zeros()).intensity().value()
}
