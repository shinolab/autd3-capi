use autd3capi_driver::{
    autd3::gain::{Bessel, BesselOption},
    driver::{
        defined::Angle,
        geometry::{UnitVector3, Vector3},
    },
    *,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainBessel(
    pos: Point3,
    dir: Vector3,
    theta: Angle,
    option: BesselOption,
) -> GainPtr {
    Bessel {
        pos,
        dir: UnitVector3::new_normalize(dir),
        theta,
        option,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainBesselIsDefault(option: BesselOption) -> bool {
    option == Default::default()
}
