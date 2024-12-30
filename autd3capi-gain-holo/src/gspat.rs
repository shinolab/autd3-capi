use std::num::NonZeroUsize;

use crate::{create_holo, BackendPtr, EmissionConstraintWrap};
use autd3_gain_holo::*;
use autd3capi_driver::{
    driver::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATSphere(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    repeat_nonzero: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(GSPAT, NalgebraBackend, Sphere, backend, points, amps, size)
        .with_repeat(NonZeroUsize::new_unchecked(repeat_nonzero as _))
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATT4010A1(
    backend: BackendPtr,
    points: *const Point3,
    amps: *const f32,
    size: u32,
    repeat_nonzero: u32,
    constraint: EmissionConstraintWrap,
) -> GainPtr {
    create_holo!(GSPAT, NalgebraBackend, T4010A1, backend, points, amps, size)
        .with_repeat(NonZeroUsize::new_unchecked(repeat_nonzero as _))
        .with_constraint(constraint.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGSPATIsDefault(
    constraint: EmissionConstraintWrap,
    repeat: u32,
) -> bool {
    let default = GSPAT::new(std::sync::Arc::new(NalgebraBackend::default()), []);
    let constraint: EmissionConstraint = constraint.into();
    constraint == default.constraint() && repeat as usize == default.repeat().get()
}
