use autd3capi_driver::{
    ResultU16,
    driver::{common::ULTRASOUND_PERIOD_COUNT_BITS, firmware::fpga::PulseWidth},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PulseWidthOutOfRange;

impl std::error::Error for PulseWidthOutOfRange {}

impl std::fmt::Display for PulseWidthOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pulse width out of range. Must be between 0 and 511")
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidth(value: u16) -> ResultU16 {
    PulseWidth::<u16, ULTRASOUND_PERIOD_COUNT_BITS>::new(value)
        .ok_or(PulseWidthOutOfRange {})
        .map(|p| p.pulse_width())
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidthFromDuty(duty: f32) -> ResultU16 {
    PulseWidth::<u16, ULTRASOUND_PERIOD_COUNT_BITS>::from_duty(duty)
        .ok_or(PulseWidthOutOfRange {})
        .map(|p| p.pulse_width())
        .into()
}
