use std::num::NonZeroUsize;

use crate::{BackendPtr, EmissionConstraintWrap, create_holo};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[repr(C)]
pub struct GSOption {
    pub constraint: EmissionConstraintWrap,
    pub repeat: u32,
}

impl From<GSOption> for autd3_gain_holo::GSOption {
    fn from(option: GSOption) -> Self {
        autd3_gain_holo::GSOption {
            constraint: option.constraint.into(),
            repeat: NonZeroUsize::new(option.repeat as _).unwrap(),
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSSphere(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GSOption,
) -> GainPtr {
    let (foci, backend) = unsafe { create_holo!(NalgebraBackend, backend, points, amps, size) };
    GS::<Sphere, NalgebraBackend> {
        foci,
        backend,
        option: option.into(),
        directivity: std::marker::PhantomData,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGST4010A1(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GSOption,
) -> GainPtr {
    let (foci, backend) = unsafe { create_holo!(NalgebraBackend, backend, points, amps, size) };
    GS::<T4010A1, NalgebraBackend> {
        foci,
        backend,
        option: option.into(),
        directivity: std::marker::PhantomData,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainGSIsDefault(option: GSOption) -> bool {
    autd3_gain_holo::GSOption::default() == option.into()
}
