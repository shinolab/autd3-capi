use autd3capi_def::driver::{
    defined::float,
    geometry::{EulerAngle, Rad, UnitQuaternion},
};

#[no_mangle]
pub unsafe extern "C" fn AUTDRotationFromEulerZYZ(x: float, y: float, z: float, rot: *mut float) {
    let r = UnitQuaternion::from(EulerAngle::ZYZ(x * Rad, y * Rad, z * Rad));
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}
