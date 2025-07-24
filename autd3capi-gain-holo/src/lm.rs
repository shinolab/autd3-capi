#![allow(clippy::missing_safety_doc)]

use std::num::NonZeroUsize;

use crate::{BackendPtr, EmissionConstraintWrap, create_holo};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[repr(C)]
pub struct LMOption {
    pub constraint: EmissionConstraintWrap,
    pub eps_1: f32,
    pub eps_2: f32,
    pub tau: f32,
    pub k_max: u32,
    pub initial: *const f32,
    pub initial_len: u32,
}

impl From<LMOption> for autd3_gain_holo::LMOption {
    fn from(option: LMOption) -> Self {
        autd3_gain_holo::LMOption {
            constraint: option.constraint.into(),
            eps_1: option.eps_1,
            eps_2: option.eps_2,
            tau: option.tau,
            k_max: NonZeroUsize::new(option.k_max as _).unwrap(),
            initial: unsafe { vec_from_raw!(option.initial, f32, option.initial_len) },
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMSphere(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: LMOption,
) -> GainPtr {
    let (foci, backend) = unsafe { create_holo!(NalgebraBackend, backend, points, amps, size) };
    LM::<Sphere, NalgebraBackend> {
        foci,
        backend,
        option: option.into(),
        directivity: std::marker::PhantomData,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMT4010A1(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: LMOption,
) -> GainPtr {
    let (foci, backend) = unsafe { create_holo!(NalgebraBackend, backend, points, amps, size) };
    LM::<T4010A1, NalgebraBackend> {
        foci,
        backend,
        option: option.into(),
        directivity: std::marker::PhantomData,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainLMIsDefault(option: LMOption) -> bool {
    autd3_gain_holo::LMOption::default() == option.into()
}
