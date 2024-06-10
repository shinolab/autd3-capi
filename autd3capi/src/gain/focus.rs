use autd3capi_driver::{
    autd3::{derive::Phase, gain::Focus},
    driver::geometry::Vector3,
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocus(p: Vector3, intensity: u8, phase_offset: u8) -> GainPtr {
    Focus::new(p)
        .with_intensity(intensity)
        .with_phase_offset(Phase::new(phase_offset))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocusIsDefault(focus: GainPtr) -> bool {
    let g = take_gain!(focus, Focus);
    let default = Focus::new(Vector3::zeros());
    g.intensity() == default.intensity() && g.phase_offset() == default.phase_offset()
}
