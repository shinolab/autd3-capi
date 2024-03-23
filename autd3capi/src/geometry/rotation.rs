use autd3capi_def::driver::geometry::{EulerAngle, Rad, UnitQuaternion};

#[no_mangle]
pub unsafe extern "C" fn AUTDRotationFromEulerZYZ(x: f64, y: f64, z: f64, rot: *mut f64) {
    let r = UnitQuaternion::from(EulerAngle::ZYZ(x * Rad, y * Rad, z * Rad));
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}
