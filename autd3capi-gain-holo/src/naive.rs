#![allow(clippy::missing_safety_doc)]

use crate::{EmissionConstraintWrap, create_holo};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[repr(C)]
pub struct NaiveOption {
    pub constraint: EmissionConstraintWrap,
}

impl From<NaiveOption> for autd3_gain_holo::NaiveOption {
    fn from(option: NaiveOption) -> Self {
        autd3_gain_holo::NaiveOption {
            constraint: option.constraint.into(),
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaiveSphere(
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: NaiveOption,
) -> GainPtr {
    Naive::<Sphere> {
        foci: unsafe { create_holo!(points, amps, size) },
        option: option.into(),
        directivity: std::marker::PhantomData,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaiveT4010A1(
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: NaiveOption,
) -> GainPtr {
    Naive::<T4010A1> {
        foci: unsafe { create_holo!(points, amps, size) },
        option: option.into(),
        directivity: std::marker::PhantomData,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainNaiveIsDefault(option: NaiveOption) -> bool {
    autd3_gain_holo::NaiveOption::default() == option.into()
}
