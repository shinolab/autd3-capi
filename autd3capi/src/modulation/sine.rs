/*
 * File: sine.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/01/2024
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::modulation::Sine, *};

use super::SamplingMode;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSine(freq: float) -> ModulationPtr {
    ModulationPtr::new(Sine::new(freq))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithSamplingConfig(
    m: ModulationPtr,
    config: SamplingConfiguration,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_sampling_config(config.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithIntensity(
    m: ModulationPtr,
    intensity: u8,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_intensity(intensity))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithPhase(
    m: ModulationPtr,
    phase: float,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_phase(phase))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithOffset(
    m: ModulationPtr,
    offset: u8,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_offset(offset))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithMode(
    m: ModulationPtr,
    mode: SamplingMode,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_mode(mode.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modulation::AUTDModulationIntoDatagram;
    use crate::tests::create_controller;
    use crate::*;

    #[test]
    fn test_sine_with_small_freq_div() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSine(150.0);
            let m = AUTDModulationSineWithSamplingConfig(
                m,
                autd3_driver::common::SamplingConfiguration::from_frequency_division(512)
                    .unwrap()
                    .into(),
            );
            let d1 = AUTDModulationIntoDatagram(m);
            let res = AUTDControllerSend(cnt, d1, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(res.result, AUTD3_ERR);
        }
    }
}
