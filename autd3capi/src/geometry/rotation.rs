use autd3capi_driver::{
    autd3::derive::rad,
    driver::geometry::{EulerAngle, UnitQuaternion},
};

#[no_mangle]
pub unsafe extern "C" fn AUTDRotationFromEulerZYZ(x: f32, y: f32, z: f32, rot: *mut f32) {
    let r = UnitQuaternion::from(EulerAngle::ZYZ(x * rad, y * rad, z * rad));
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}
