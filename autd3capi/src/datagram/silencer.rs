use std::{num::NonZeroU8, time::Duration};

use autd3capi_driver::{
    autd3::derive::SamplingConfig,
    driver::datagram::{Silencer, SilencerFixedCompletionTime, SilencerFixedUpdateRate},
    take, DatagramPtr, SilencerTarget,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromUpdateRate(
    value_intensity: u8,
    value_phase: u8,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer::from_update_rate(
        NonZeroU8::new_unchecked(value_intensity),
        NonZeroU8::new_unchecked(value_phase),
    )
    .with_target(target.into())
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedUpdateRateIsValid(
    silencer: DatagramPtr,
    config_intensity: SamplingConfig,
    config_phase: SamplingConfig,
) -> bool {
    take!(silencer, Box<SilencerFixedUpdateRate>).is_valid(&(config_intensity, config_phase))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromCompletionTime(
    value_intensity: u64,
    value_phase: u64,
    strict_mode: bool,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer::from_completion_time(
        Duration::from_nanos(value_intensity),
        Duration::from_nanos(value_phase),
    )
    .with_strict_mode(strict_mode)
    .with_target(target.into())
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionTimeIsValid(
    silencer: DatagramPtr,
    config_intensity: SamplingConfig,
    config_phase: SamplingConfig,
) -> bool {
    take!(silencer, Box<SilencerFixedCompletionTime>).is_valid(&(config_intensity, config_phase))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionTimeIsDefault(
    silencer: DatagramPtr,
) -> bool {
    let silencer = take!(silencer, Box<SilencerFixedCompletionTime>);
    let default = SilencerFixedCompletionTime::default();
    silencer.completion_time_intensity() == default.completion_time_intensity()
        && silencer.completion_time_phase() == default.completion_time_phase()
        && silencer.strict_mode() == default.strict_mode()
        && silencer.target() == default.target()
}
