use std::num::NonZeroU16;

use autd3capi_driver::{
    autd3::{derive::SamplingConfig, prelude::SilencerTarget},
    driver::datagram::{FixedCompletionSteps, FixedUpdateRate, HasSamplingConfig, Silencer},
    DatagramPtr,
};

#[cfg(not(feature = "dynamic_freq"))]
use autd3capi_driver::Duration;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromUpdateRate(
    intensity: u16,
    phase: u16,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer::new(FixedUpdateRate {
        intensity: NonZeroU16::new_unchecked(intensity),
        phase: NonZeroU16::new_unchecked(phase),
    })
    .with_target(target)
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromCompletionSteps(
    intensity: u16,
    phase: u16,
    strict_mode: bool,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer::new(FixedCompletionSteps {
        intensity: NonZeroU16::new_unchecked(intensity),
        phase: NonZeroU16::new_unchecked(phase),
    })
    .with_strict_mode(strict_mode)
    .with_target(target)
    .into()
}

#[cfg(not(feature = "dynamic_freq"))]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromCompletionTime(
    intensity: Duration,
    phase: Duration,
    strict_mode: bool,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer::new(autd3capi_driver::autd3::prelude::FixedCompletionTime {
        intensity: intensity.into(),
        phase: phase.into(),
    })
    .with_strict_mode(strict_mode)
    .with_target(target)
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionStepsIsValid(
    intensity: u16,
    phase: u16,
    strict_mode: bool,
    config_intensity: SamplingConfig,
    config_phase: SamplingConfig,
) -> bool {
    Silencer::new(FixedCompletionSteps {
        intensity: NonZeroU16::new_unchecked(intensity),
        phase: NonZeroU16::new_unchecked(phase),
    })
    .with_strict_mode(strict_mode)
    .is_valid(&SamplingConfigTuple(config_intensity, config_phase))
}

#[cfg(not(feature = "dynamic_freq"))]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionTimeIsValid(
    intensity: Duration,
    phase: Duration,
    strict_mode: bool,
    config_intensity: SamplingConfig,
    config_phase: SamplingConfig,
) -> bool {
    Silencer::new(autd3capi_driver::autd3::prelude::FixedCompletionTime {
        intensity: intensity.into(),
        phase: phase.into(),
    })
    .with_strict_mode(strict_mode)
    .is_valid(&SamplingConfigTuple(config_intensity, config_phase))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionStepsIsDefault(
    intensity: u16,
    phase: u16,
    strict_mode: bool,
    target: SilencerTarget,
) -> bool {
    let default = Silencer::default();
    intensity == default.config().intensity.get()
        && phase == default.config().phase.get()
        && strict_mode == default.strict_mode()
        && target == default.target()
}

struct SamplingConfigTuple(SamplingConfig, SamplingConfig);

impl HasSamplingConfig for SamplingConfigTuple {
    fn intensity(&self) -> Option<SamplingConfig> {
        Some(self.0)
    }

    fn phase(&self) -> Option<SamplingConfig> {
        Some(self.1)
    }
}
