/*
 * File: constraint.rs
 * Project: src
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use crate::EmissionConstraintPtr;
use autd3_gain_holo::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintDotCare() -> EmissionConstraintPtr {
    EmissionConstraintPtr(Box::into_raw(Box::new(EmissionConstraint::DontCare)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintNormalize() -> EmissionConstraintPtr {
    EmissionConstraintPtr(Box::into_raw(Box::new(EmissionConstraint::Normalize)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintUniform(intensity: u8) -> EmissionConstraintPtr {
    EmissionConstraintPtr(
        Box::into_raw(Box::new(EmissionConstraint::Uniform(intensity.into()))) as _,
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintClamp(
    min_v: u8,
    max_v: u8,
) -> EmissionConstraintPtr {
    EmissionConstraintPtr(Box::into_raw(Box::new(EmissionConstraint::Clamp(
        min_v.into(),
        max_v.into(),
    ))) as _)
}
