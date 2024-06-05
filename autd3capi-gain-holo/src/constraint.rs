#![allow(clippy::missing_safety_doc)]

use autd3_gain_holo::*;

#[repr(u8)]
enum EmissionConstraintTag {
    DontCare = 0,
    Normalize = 1,
    Uniform = 2,
    Multiply = 3,
    Clamp = 4,
}

#[repr(C)]
union EmissionConstraintValue {
    null: u8,
    uniform: u8,
    multiply: f32,
    clamp: [u8; 2],
}

#[repr(C)]
pub struct EmissionConstraintWrap {
    tag: EmissionConstraintTag,
    value: EmissionConstraintValue,
}

impl From<EmissionConstraintWrap> for EmissionConstraint {
    fn from(value: EmissionConstraintWrap) -> Self {
        match value.tag {
            EmissionConstraintTag::DontCare => EmissionConstraint::DontCare,
            EmissionConstraintTag::Normalize => EmissionConstraint::Normalize,
            EmissionConstraintTag::Uniform => {
                EmissionConstraint::Uniform(unsafe { value.value.uniform.into() })
            }
            EmissionConstraintTag::Multiply => {
                EmissionConstraint::Multiply(unsafe { value.value.multiply })
            }
            EmissionConstraintTag::Clamp => {
                EmissionConstraint::Clamp(unsafe { value.value.clamp[0].into() }, unsafe {
                    value.value.clamp[1].into()
                })
            }
        }
    }
}

impl From<EmissionConstraint> for EmissionConstraintWrap {
    fn from(value: EmissionConstraint) -> Self {
        match value {
            EmissionConstraint::DontCare => EmissionConstraintWrap {
                tag: EmissionConstraintTag::DontCare,
                value: EmissionConstraintValue { null: 0 },
            },
            EmissionConstraint::Normalize => EmissionConstraintWrap {
                tag: EmissionConstraintTag::Normalize,
                value: EmissionConstraintValue { null: 0 },
            },
            EmissionConstraint::Uniform(v) => EmissionConstraintWrap {
                tag: EmissionConstraintTag::Uniform,
                value: EmissionConstraintValue { uniform: v.value() },
            },
            EmissionConstraint::Multiply(v) => EmissionConstraintWrap {
                tag: EmissionConstraintTag::Multiply,
                value: EmissionConstraintValue { multiply: v },
            },
            EmissionConstraint::Clamp(min, max) => EmissionConstraintWrap {
                tag: EmissionConstraintTag::Clamp,
                value: EmissionConstraintValue {
                    clamp: [min.value(), max.value()],
                },
            },
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintDotCare() -> EmissionConstraintWrap {
    autd3_gain_holo::EmissionConstraint::DontCare.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintNormalize() -> EmissionConstraintWrap {
    autd3_gain_holo::EmissionConstraint::Normalize.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintUniform(intensity: u8) -> EmissionConstraintWrap {
    autd3_gain_holo::EmissionConstraint::Uniform(intensity.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintMultiply(v: f32) -> EmissionConstraintWrap {
    autd3_gain_holo::EmissionConstraint::Multiply(v).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintClamp(
    min_v: u8,
    max_v: u8,
) -> EmissionConstraintWrap {
    autd3_gain_holo::EmissionConstraint::Clamp(min_v.into(), max_v.into()).into()
}
