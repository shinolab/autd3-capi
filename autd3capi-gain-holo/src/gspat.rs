use std::num::NonZeroUsize;

use crate::{create_holo, BackendPtr, EmissionConstraintWrap};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Directivity, Sphere, T4010A1},
    *,
};

#[repr(C)]
pub struct GSPATOption {
    pub constraint: EmissionConstraintWrap,
    pub repeat: u32,
}

impl<T: Directivity> From<GSPATOption> for autd3_gain_holo::GSPATOption<T> {
    fn from(option: GSPATOption) -> Self {
        autd3_gain_holo::GSPATOption {
            constraint: option.constraint.into(),
            repeat: NonZeroUsize::new(option.repeat as _).unwrap(),
            __phantom: std::marker::PhantomData,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATSphere(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GSPATOption,
) -> GainPtr {
    let (foci, backend) = create_holo!(NalgebraBackend, Sphere, backend, points, amps, size);
    GSPAT {
        foci,
        backend,
        option: option.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATT4010A1(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GSPATOption,
) -> GainPtr {
    let (foci, backend) = create_holo!(NalgebraBackend, T4010A1, backend, points, amps, size);
    GSPAT {
        foci,
        backend,
        option: option.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGSPATIsDefault(option: GSPATOption) -> bool {
    autd3_gain_holo::GSPATOption::<Sphere>::default() == option.into()
}
