use autd3capi_driver::driver::{
    common::rad,
    geometry::{EulerAngle, Quaternion, UnitQuaternion},
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDRotationFromEulerXYZ(x: f32, y: f32, z: f32) -> Quaternion {
    UnitQuaternion::from(EulerAngle::XYZ(x * rad, y * rad, z * rad)).quaternion()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDRotationFromEulerZYZ(z1: f32, y: f32, z2: f32) -> Quaternion {
    UnitQuaternion::from(EulerAngle::ZYZ(z1 * rad, y * rad, z2 * rad)).quaternion()
}
