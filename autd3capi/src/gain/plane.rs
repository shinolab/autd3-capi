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
    phase_offset: u8,
) -> GainPtr {
    Plane::new(Vector3::new(nx, ny, nz))
        .with_intensity(intensity)
        .with_phase_offset(Phase::from(phase_offset))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlanelIsDefault(plane: GainPtr) -> bool {
    let g = take_gain!(plane, Plane);
    let default = Plane::new(Vector3::zeros());
    g.intensity() == default.intensity() && g.phase_offset() == default.phase_offset()
}
