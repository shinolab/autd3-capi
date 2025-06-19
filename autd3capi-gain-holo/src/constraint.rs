#![allow(clippy::missing_safety_doc)]

use autd3_gain_holo::*;
use autd3capi_driver::autd3::core::gain::Intensity;

#[repr(u8)]
pub enum EmissionConstraintTag {
    Normalize = 1,
    Uniform = 2,
    Multiply = 3,
    Clamp = 4,
}

#[repr(C)]
pub union EmissionConstraintValue {
    null: Intensity,
    uniform: Intensity,
    multiply: f32,
    clamp: [Intensity; 2],
}

#[repr(C)]
pub struct EmissionConstraintWrap {
    tag: EmissionConstraintTag,
    value: EmissionConstraintValue,
}

impl From<EmissionConstraintWrap> for EmissionConstraint {
    fn from(value: EmissionConstraintWrap) -> Self {
        unsafe {
            match value.tag {
                EmissionConstraintTag::Normalize => EmissionConstraint::Normalize,
                EmissionConstraintTag::Uniform => EmissionConstraint::Uniform(value.value.uniform),
                EmissionConstraintTag::Multiply => {
                    EmissionConstraint::Multiply(value.value.multiply)
                }
                EmissionConstraintTag::Clamp => {
                    EmissionConstraint::Clamp(value.value.clamp[0], value.value.clamp[1])
                }
            }
        }
    }
}

impl From<EmissionConstraint> for EmissionConstraintWrap {
    fn from(value: EmissionConstraint) -> Self {
        match value {
            EmissionConstraint::Normalize => EmissionConstraintWrap {
                tag: EmissionConstraintTag::Normalize,
                value: EmissionConstraintValue {
                    null: Intensity::MIN,
                },
            },
            EmissionConstraint::Uniform(v) => EmissionConstraintWrap {
                tag: EmissionConstraintTag::Uniform,
                value: EmissionConstraintValue { uniform: v },
            },
            EmissionConstraint::Multiply(v) => EmissionConstraintWrap {
                tag: EmissionConstraintTag::Multiply,
                value: EmissionConstraintValue { multiply: v },
            },
            EmissionConstraint::Clamp(min, max) => EmissionConstraintWrap {
                tag: EmissionConstraintTag::Clamp,
                value: EmissionConstraintValue { clamp: [min, max] },
            },
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintNormalize() -> EmissionConstraintWrap {
    autd3_gain_holo::EmissionConstraint::Normalize.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintUniform(
    intensity: Intensity,
) -> EmissionConstraintWrap {
    autd3_gain_holo::EmissionConstraint::Uniform(intensity).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintMultiply(v: f32) -> EmissionConstraintWrap {
    autd3_gain_holo::EmissionConstraint::Multiply(v).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintClamp(
    min_v: Intensity,
    max_v: Intensity,
) -> EmissionConstraintWrap {
    autd3_gain_holo::EmissionConstraint::Clamp(min_v, max_v).into()
}
