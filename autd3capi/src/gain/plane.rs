use autd3::prelude::Phase;
use autd3capi_driver::{autd3::gain::Plane, driver::geometry::Vector3, *};
use driver::geometry::UnitVector3;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlane(n: Vector3, intensity: u8, phase_offset: u8) -> GainPtr {
    Plane::new(UnitVector3::new_normalize(n))
        .with_intensity(intensity)
        .with_phase_offset(Phase::from(phase_offset))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlanelIsDefault(intensity: u8, phase_offset: u8) -> bool {
    let default = Plane::new(Vector3::x_axis());
    intensity == default.intensity().value() && phase_offset == default.phase_offset().value()
}
