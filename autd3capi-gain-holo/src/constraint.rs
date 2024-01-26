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
    EmissionConstraint::Uniform(intensity.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintClamp(
    min_v: u8,
    max_v: u8,
) -> EmissionConstraintPtr {
    EmissionConstraint::Clamp(min_v.into(), max_v.into()).into()
}
