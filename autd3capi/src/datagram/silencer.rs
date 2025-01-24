use autd3capi_driver::{
    driver::{
        datagram::{FixedCompletionSteps, FixedUpdateRate, Silencer},
        firmware::fpga::SilencerTarget,
    },
    *,
};

#[cfg(not(feature = "dynamic_freq"))]
use autd3capi_driver::Duration;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromUpdateRate(
    config: FixedUpdateRate,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer { config, target }.into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFromCompletionSteps(
    config: FixedCompletionSteps,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer { config, target }.into()
}

#[repr(C)]
#[cfg(not(feature = "dynamic_freq"))]
pub struct FixedCompletionTime {
    pub intensity: Duration,
    pub phase: Duration,
    pub strict_mode: bool,
}

#[cfg(not(feature = "dynamic_freq"))]
impl From<FixedCompletionTime> for autd3::driver::datagram::FixedCompletionTime {
    fn from(config: FixedCompletionTime) -> Self {
        autd3::driver::datagram::FixedCompletionTime {
            intensity: config.intensity.into(),
            phase: config.phase.into(),
            strict_mode: config.strict_mode,
        }
    }
}

#[no_mangle]
#[must_use]
#[cfg(not(feature = "dynamic_freq"))]
pub unsafe extern "C" fn AUTDDatagramSilencerFromCompletionTime(
    config: FixedCompletionTime,
    target: SilencerTarget,
) -> DatagramPtr {
    Silencer::<autd3::driver::datagram::FixedCompletionTime> {
        config: config.into(),
        target,
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionStepsIsDefault(
    config: FixedCompletionSteps,
    target: SilencerTarget,
) -> bool {
    Silencer { config, target } == Default::default()
}
