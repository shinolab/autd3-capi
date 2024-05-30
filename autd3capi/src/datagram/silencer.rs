use autd3capi_driver::{
    driver::datagram::{Silencer, SilencerFixedCompletionSteps},
    take, DatagramPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedUpdateRate(
    value_intensity: u16,
    value_phase: u16,
) -> DatagramPtr {
    Silencer::fixed_update_rate(value_intensity, value_phase).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionSteps(
    value_intensity: u16,
    value_phase: u16,
    strict_mode: bool,
) -> DatagramPtr {
    Silencer::fixed_completion_steps(value_intensity, value_phase)
        .with_strict_mode(strict_mode)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionStepsIsDefault(
    silencer: DatagramPtr,
) -> bool {
    let silencer = take!(silencer, Box<SilencerFixedCompletionSteps>);
    let default = SilencerFixedCompletionSteps::default();
    silencer.completion_steps_intensity() == default.completion_steps_intensity()
        && silencer.completion_steps_phase() == default.completion_steps_phase()
        && silencer.strict_mode() == default.strict_mode()
}
