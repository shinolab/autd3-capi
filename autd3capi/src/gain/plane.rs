use autd3::gain::PlaneOption;
use autd3capi_driver::{autd3::gain::Plane, driver::geometry::Vector3, *};
use driver::geometry::UnitVector3;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlane(n: Vector3, option: PlaneOption) -> GainPtr {
    Plane {
        dir: UnitVector3::new_normalize(n),
        option,
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlanelIsDefault(option: PlaneOption) -> bool {
    option == Default::default()
}
