use autd3::prelude::{rad, Phase};
use autd3capi_driver::{autd3::gain::Bessel, driver::geometry::Vector3, *};
use driver::geometry::UnitVector3;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBessel(
    p: Vector3,
    n: Vector3,
    theta_z: f32,
    intensity: u8,
    phase_offset: u8,
) -> GainPtr {
    Bessel::new(p, UnitVector3::new_normalize(n), theta_z * rad)
        .with_intensity(intensity)
        .with_phase_offset(Phase::new(phase_offset))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBesselIsDefault(intensity: u8, phase_offset: u8) -> bool {
    let default = Bessel::new(Vector3::zeros(), Vector3::x_axis(), 0.0 * rad);
    intensity == default.intensity().value() && phase_offset == default.phase_offset().value()
}
