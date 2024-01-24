use autd3capi_def::{
    autd3::gain::Plane,
    driver::{derive::Phase, geometry::Vector3},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlane(
    nx: float,
    ny: float,
    nz: float,
    intensity: u8,
    phase: u8,
) -> GainPtr {
    Plane::new(Vector3::new(nx, ny, nz))
        .with_intensity(intensity)
        .with_phase(Phase::from(phase))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlaneDefaultIntensity() -> u8 {
    Plane::new(Vector3::zeros()).intensity().value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlaneDefaultPhase() -> u8 {
    Plane::new(Vector3::zeros()).phase().value()
}
