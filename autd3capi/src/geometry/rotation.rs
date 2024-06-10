use autd3capi_driver::{
    autd3::derive::rad,
    driver::geometry::{EulerAngle, Quaternion, UnitQuaternion},
};

#[no_mangle]
pub unsafe extern "C" fn AUTDRotationFromEulerZYZ(x: f32, y: f32, z: f32) -> Quaternion {
    *UnitQuaternion::from(EulerAngle::ZYZ(x * rad, y * rad, z * rad)).quaternion()
}
