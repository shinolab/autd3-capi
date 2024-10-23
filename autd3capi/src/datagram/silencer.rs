use std::{num::NonZeroU16, time::Duration};

use autd3capi_driver::{
    autd3::{derive::SamplingConfig, prelude::SilencerTarget},
    driver::datagram::{FixedCompletionTime, FixedUpdateRate, Silencer, WithSampling},
    take, DatagramPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromUpdateRate(
    value_intensity: u16,
    value_phase: u16,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer::new(FixedUpdateRate {
        intensity: NonZeroU16::new_unchecked(value_intensity),
        phase: NonZeroU16::new_unchecked(value_phase),
    })
    .with_target(target)
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedUpdateRateIsValid(
    silencer: DatagramPtr,
    config_intensity: SamplingConfig,
    config_phase: SamplingConfig,
) -> bool {
    take!(silencer, Box<Silencer<FixedUpdateRate>>)
        .is_valid(&SamplingConfigTuple(config_intensity, config_phase))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromCompletionTime(
    value_intensity: u64,
    value_phase: u64,
    strict_mode: bool,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer::new(FixedCompletionTime {
        intensity: Duration::from_nanos(value_intensity),
        phase: Duration::from_nanos(value_phase),
    })
    .with_strict_mode(strict_mode)
    .with_target(target)
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionTimeIsValid(
    silencer: DatagramPtr,
    config_intensity: SamplingConfig,
    config_phase: SamplingConfig,
) -> bool {
    take!(silencer, Box<Silencer<FixedCompletionTime>>)
        .is_valid(&SamplingConfigTuple(config_intensity, config_phase))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionTimeIsDefault(
    silencer: DatagramPtr,
) -> bool {
    let silencer = take!(silencer, Box<Silencer<FixedCompletionTime>>);
    let default = Silencer::default();
    silencer.config().intensity() == default.config().intensity()
        && silencer.config().phase() == default.config().phase()
        && silencer.strict_mode() == default.strict_mode()
        && silencer.target() == default.target()
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
