#![allow(clippy::missing_safety_doc)]

use autd3_gain_holo::*;
use autd3capi_def::ConstPtr;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EmissionConstraintPtr(pub ConstPtr);

impl From<EmissionConstraint> for EmissionConstraintPtr {
    fn from(c: EmissionConstraint) -> Self {
        Self(Box::into_raw(Box::new(c)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintDotCare() -> EmissionConstraintPtr {
    EmissionConstraint::DontCare.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintNormalize() -> EmissionConstraintPtr {
    EmissionConstraint::Normalize.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintUniform(intensity: u8) -> EmissionConstraintPtr {
    EmissionConstraintPtr(
        Box::into_raw(Box::new(EmissionConstraint::Uniform(intensity.into()))) as _,
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintClamp(
    min_v: u8,
    max_v: u8,
) -> EmissionConstraintPtr {
    EmissionConstraintPtr(Box::into_raw(Box::new(EmissionConstraint::Clamp(
        min_v.into(),
        max_v.into(),
    ))) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintEq(
    a: EmissionConstraintPtr,
    b: EmissionConstraintPtr,
) -> bool {
    *Box::from_raw(a.0 as *mut EmissionConstraint) == *Box::from_raw(b.0 as *mut EmissionConstraint)
}
