use autd3::{gain::BesselOption, prelude::rad};
use autd3capi_driver::{autd3::gain::Bessel, driver::geometry::Vector3, *};
use driver::geometry::UnitVector3;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBessel(
    pos: Point3,
    dir: Vector3,
    theta: f32,
    option: BesselOption,
) -> GainPtr {
    Bessel {
        pos,
        dir: UnitVector3::new_normalize(dir),
        theta: theta * rad,
        option,
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainBesselIsDefault(option: BesselOption) -> bool {
    option == Default::default()
}
