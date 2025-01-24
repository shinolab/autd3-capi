#![allow(clippy::missing_safety_doc)]

use crate::{create_holo, BackendPtr, EmissionConstraintWrap};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Directivity, Sphere, T4010A1},
    *,
};

#[repr(C)]
pub struct NaiveOption {
    pub constraint: EmissionConstraintWrap,
}

impl<T: Directivity> From<NaiveOption> for autd3_gain_holo::NaiveOption<T> {
    fn from(option: NaiveOption) -> Self {
        autd3_gain_holo::NaiveOption {
            constraint: option.constraint.into(),
            __phantom: std::marker::PhantomData,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaiveSphere(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: NaiveOption,
) -> GainPtr {
    let (foci, backend) = create_holo!(NalgebraBackend, Sphere, backend, points, amps, size);
    Naive {
        foci,
        backend,
        option: option.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaiveT4010A1(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: NaiveOption,
) -> GainPtr {
    let (foci, backend) = create_holo!(NalgebraBackend, T4010A1, backend, points, amps, size);
    Naive {
        foci,
        backend,
        option: option.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainNaiveIsDefault(option: NaiveOption) -> bool {
    autd3_gain_holo::NaiveOption::<Sphere>::default() == option.into()
}
