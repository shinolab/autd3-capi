use autd3capi_def::{
    driver::datagram::{ConfigureSilencer, ConfigureSilencerFixedCompletionSteps},
    take, DatagramPtr, ResultDatagram,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedUpdateRate(
    value_intensity: u16,
    value_phase: u16,
) -> ResultDatagram {
    ConfigureSilencer::fixed_update_rate(value_intensity, value_phase).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionSteps(
    value_intensity: u16,
    value_phase: u16,
    strict_mode: bool,
) -> ResultDatagram {
    ConfigureSilencer::fixed_completion_steps(value_intensity, value_phase)
        .map(|s| s.with_strict_mode(strict_mode))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionStepsIsDefault(
    silencer: DatagramPtr,
) -> bool {
    let silencer = take!(silencer, ConfigureSilencerFixedCompletionSteps);
    silencer.strict_mode() == ConfigureSilencerFixedCompletionSteps::default().strict_mode()
}
