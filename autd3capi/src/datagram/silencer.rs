use autd3capi_driver::{
    driver::datagram::{Silencer, SilencerFixedCompletionSteps},
    take, DatagramPtr, ResultDatagram,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedUpdateRate(
    value_intensity: u16,
    value_phase: u16,
) -> ResultDatagram {
    Silencer::fixed_update_rate(value_intensity, value_phase).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionSteps(
    value_intensity: u16,
    value_phase: u16,
    strict_mode: bool,
) -> ResultDatagram {
    Silencer::fixed_completion_steps(value_intensity, value_phase)
        .map(|s| s.with_strict_mode(strict_mode))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_default() {
        unsafe {
            let silencer = AUTDDatagramSilencerFixedCompletionSteps(10, 40, true).result;
            assert!(AUTDDatagramSilencerFixedCompletionStepsIsDefault(silencer));

            let silencer = AUTDDatagramSilencerFixedCompletionSteps(10, 40, false).result;
            assert!(!AUTDDatagramSilencerFixedCompletionStepsIsDefault(silencer));

            let silencer = AUTDDatagramSilencerFixedCompletionSteps(11, 40, true).result;
            assert!(!AUTDDatagramSilencerFixedCompletionStepsIsDefault(silencer));

            let silencer = AUTDDatagramSilencerFixedCompletionSteps(10, 41, true).result;
            assert!(!AUTDDatagramSilencerFixedCompletionStepsIsDefault(silencer));
        }
    }
}
