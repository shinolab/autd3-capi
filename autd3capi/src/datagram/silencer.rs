use std::num::NonZeroU16;

use autd3capi_driver::{
    autd3::{derive::SamplingConfig, prelude::SilencerTarget},
    driver::datagram::{FixedCompletionTime, FixedUpdateRate, Silencer, WithSampling},
    DatagramPtr, Duration,
};

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
pub unsafe extern "C" fn AUTDDatagramSilencerFixedUpdateRateIsValid(
    intensity: u16,
    phase: u16,
    config_intensity: SamplingConfig,
    config_phase: SamplingConfig,
) -> bool {
    Silencer::new(FixedUpdateRate {
        intensity: NonZeroU16::new_unchecked(intensity),
        phase: NonZeroU16::new_unchecked(phase),
    })
    .is_valid(&SamplingConfigTuple(config_intensity, config_phase))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromCompletionTime(
    intensity: Duration,
    phase: Duration,
    strict_mode: bool,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer::new(FixedCompletionTime {
        intensity: intensity.into(),
        phase: phase.into(),
    })
    .with_strict_mode(strict_mode)
    .with_target(target)
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionTimeIsValid(
    intensity: Duration,
    phase: Duration,
    strict_mode: bool,
    config_intensity: SamplingConfig,
    config_phase: SamplingConfig,
) -> bool {
    Silencer::new(FixedCompletionTime {
        intensity: intensity.into(),
        phase: phase.into(),
    })
    .with_strict_mode(strict_mode)
    .is_valid(&SamplingConfigTuple(config_intensity, config_phase))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionTimeIsDefault(
    intensity: u64,
    phase: u64,
    strict_mode: bool,
    target: SilencerTarget,
) -> bool {
    let default = Silencer::default();
    intensity as u128 == default.config().intensity().as_nanos()
        && phase as u128 == default.config().phase().as_nanos()
        && strict_mode == default.strict_mode()
        && target == default.target()
}

struct SamplingConfigTuple(SamplingConfig, SamplingConfig);

impl WithSampling for SamplingConfigTuple {
    fn sampling_config_intensity(&self) -> Option<SamplingConfig> {
        Some(self.0)
    }

    fn sampling_config_phase(&self) -> Option<SamplingConfig> {
        Some(self.1)
    }
}
