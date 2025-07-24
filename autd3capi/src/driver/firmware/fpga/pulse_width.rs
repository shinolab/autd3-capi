use autd3capi_driver::{ResultPulseWidth, ResultU16, autd3::core::datagram::PulseWidth};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidth(value: u32) -> PulseWidth {
    PulseWidth::new(value)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidthFromDuty(duty: f32) -> ResultPulseWidth {
    PulseWidth::from_duty(duty).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidthPulseWidth(
    pulse_width: PulseWidth,
    period: u32,
) -> ResultU16 {
    pulse_width.pulse_width::<u16>(period).into()
}
