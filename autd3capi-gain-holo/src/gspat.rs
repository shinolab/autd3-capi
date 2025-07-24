use std::num::NonZeroUsize;

use crate::{BackendPtr, EmissionConstraintWrap, create_holo};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[repr(C)]
pub struct GSPATOption {
    pub constraint: EmissionConstraintWrap,
    pub repeat: u32,
}

impl From<GSPATOption> for autd3_gain_holo::GSPATOption {
    fn from(option: GSPATOption) -> Self {
        autd3_gain_holo::GSPATOption {
            constraint: option.constraint.into(),
            repeat: NonZeroUsize::new(option.repeat as _).unwrap(),
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATSphere(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GSPATOption,
) -> GainPtr {
    let (foci, backend) = unsafe { create_holo!(NalgebraBackend, backend, points, amps, size) };
    GSPAT::<Sphere, NalgebraBackend> {
        foci,
        backend,
        option: option.into(),
        directivity: std::marker::PhantomData,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATT4010A1(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GSPATOption,
) -> GainPtr {
    let (foci, backend) = unsafe { create_holo!(NalgebraBackend, backend, points, amps, size) };
    GSPAT::<T4010A1, NalgebraBackend> {
        foci,
        backend,
        option: option.into(),
        directivity: std::marker::PhantomData,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainGSPATIsDefault(option: GSPATOption) -> bool {
    autd3_gain_holo::GSPATOption::default() == option.into()
}
