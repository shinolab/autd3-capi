use autd3capi_def::{autd3::gain::Bessel, driver::geometry::Vector3, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBessel(
    x: float,
    y: float,
    z: float,
    nx: float,
    ny: float,
    nz: float,
    theta_z: float,
    intensity: u8,
) -> GainPtr {
    GainPtr::new(
        Bessel::new(Vector3::new(x, y, z), Vector3::new(nx, ny, nz), theta_z)
            .with_intensity(intensity),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBesselDefaultIntensity() -> u8 {
    Bessel::new(Vector3::zeros(), Vector3::zeros(), 0.0)
        .intensity()
        .value()
}
