use std::num::NonZeroUsize;

use crate::{create_holo, BackendPtr, EmissionConstraintWrap};
use autd3_gain_holo::*;
use autd3capi_driver::{
    autd3::core::acoustics::directivity::{Directivity, Sphere, T4010A1},
    *,
};

#[repr(C)]
pub struct GSOption {
    pub constraint: EmissionConstraintWrap,
    pub repeat: u32,
}

impl<T: Directivity> From<GSOption> for autd3_gain_holo::GSOption<T> {
    fn from(option: GSOption) -> Self {
        autd3_gain_holo::GSOption {
            constraint: option.constraint.into(),
            repeat: NonZeroUsize::new(option.repeat as _).unwrap(),
            __phantom: std::marker::PhantomData,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSSphere(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GSOption,
) -> GainPtr {
    let (foci, backend) = create_holo!(NalgebraBackend, Sphere, backend, points, amps, size);
    GS {
        foci,
        backend,
        option: option.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGST4010A1(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    option: GSOption,
) -> GainPtr {
    let (foci, backend) = create_holo!(NalgebraBackend, T4010A1, backend, points, amps, size);
    GS {
        foci,
        backend,
        option: option.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGSIsDefault(option: GSOption) -> bool {
    autd3_gain_holo::GSOption::<Sphere>::default() == option.into()
}
