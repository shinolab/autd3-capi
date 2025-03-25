use autd3capi_driver::{
    Duration,
    driver::datagram::{FixedCompletionSteps, FixedUpdateRate, Silencer},
    *,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromUpdateRate(
    config: FixedUpdateRate,
) -> DatagramPtr {
    Silencer { config }.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromCompletionSteps(
    config: FixedCompletionSteps,
) -> DatagramPtr {
    Silencer { config }.into()
}

#[repr(C)]
pub struct FixedCompletionTime {
    pub intensity: Duration,
    pub phase: Duration,
    pub strict_mode: bool,
}

impl From<FixedCompletionTime> for autd3::driver::datagram::FixedCompletionTime {
    fn from(config: FixedCompletionTime) -> Self {
        autd3::driver::datagram::FixedCompletionTime {
            intensity: config.intensity.into(),
            phase: config.phase.into(),
            strict_mode: config.strict_mode,
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromCompletionTime(
    config: FixedCompletionTime,
) -> DatagramPtr {
    Silencer::<autd3::driver::datagram::FixedCompletionTime> {
        config: config.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionStepsIsDefault(
    config: FixedCompletionSteps,
) -> bool {
    Silencer { config } == Default::default()
}
