use autd3::prelude::Phase;
use autd3capi_driver::{autd3::gain::Focus, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocus(p: Point3, intensity: u8, phase_offset: u8) -> GainPtr {
    Focus::new(p)
        .with_intensity(intensity)
        .with_phase_offset(Phase::new(phase_offset))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocusIsDefault(intensity: u8, phase_offset: u8) -> bool {
    let default = Focus::new(Point3::origin());
    intensity == default.intensity().value() && phase_offset == default.phase_offset().value()
}
