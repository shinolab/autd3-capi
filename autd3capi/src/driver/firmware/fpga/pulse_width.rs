use autd3capi_driver::{ResultU8, ResultU16, autd3::core::datagram::PulseWidth};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PulseWidthOutOfRange<const PERIOD: usize>;

impl<const PERIOD: usize> std::error::Error for PulseWidthOutOfRange<PERIOD> {}

impl<const PERIOD: usize> std::fmt::Display for PulseWidthOutOfRange<PERIOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pulse width out of range. Must be between 0 and {}",
            PERIOD - 1
        )
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidth256(value: u8) -> ResultU8 {
    PulseWidth::<8, u8>::new(value)
        .ok_or(PulseWidthOutOfRange::<256>)
        .map(|p| p.pulse_width())
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidth512(value: u16) -> ResultU16 {
    PulseWidth::<9, u16>::new(value)
        .ok_or(PulseWidthOutOfRange::<512>)
        .map(|p| p.pulse_width())
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidth256FromDuty(duty: f32) -> ResultU8 {
    PulseWidth::<8, u8>::from_duty(duty)
        .ok_or(PulseWidthOutOfRange::<256>)
        .map(|p| p.pulse_width())
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidth512FromDuty(duty: f32) -> ResultU16 {
    PulseWidth::<9, u16>::from_duty(duty)
        .ok_or(PulseWidthOutOfRange::<512>)
        .map(|p| p.pulse_width())
        .into()
}
