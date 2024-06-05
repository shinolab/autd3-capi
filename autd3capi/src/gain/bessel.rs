use autd3capi_driver::{
    autd3::{
        derive::{rad, Phase},
        gain::Bessel,
    },
    driver::geometry::Vector3,
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBessel(
    x: f32,
    y: f32,
    z: f32,
    nx: f32,
    ny: f32,
    nz: f32,
    theta_z: f32,
    intensity: u8,
    phase_offset: u8,
) -> GainPtr {
    Bessel::new(
        Vector3::new(x, y, z),
        Vector3::new(nx, ny, nz),
        theta_z * rad,
    )
    .with_intensity(intensity)
    .with_phase_offset(Phase::new(phase_offset))
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBesselIsDefault(bessel: GainPtr) -> bool {
    let g = take_gain!(bessel, Bessel);
    let default = Bessel::new(Vector3::zeros(), Vector3::zeros(), 0.0 * rad);
    g.intensity() == default.intensity() && g.phase_offset() == default.phase_offset()
}
