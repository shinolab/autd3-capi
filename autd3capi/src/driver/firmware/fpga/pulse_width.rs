use autd3capi_driver::{ResultU16, autd3::core::firmware::PulseWidth};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidth(value: u16) -> PulseWidth {
    PulseWidth::new(value)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidthFromDuty(duty: f32) -> PulseWidth {
    PulseWidth::from_duty(duty)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDPulseWidthPulseWidth(pulse_width: PulseWidth) -> ResultU16 {
    pulse_width.pulse_width().into()
}
