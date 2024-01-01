/*
 * File: static.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/01/2024
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::modulation::Static, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStatic() -> ModulationPtr {
    ModulationPtr::new(Static::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStaticWithIntensity(intensity: u8) -> ModulationPtr {
    ModulationPtr::new(Static::with_intensity(intensity))
}
