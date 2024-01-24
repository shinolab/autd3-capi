#[repr(u8)]
pub enum GainSTMMode {
    PhaseIntensityFull = 0,
    PhaseFull = 1,
    PhaseHalf = 2,
}

impl From<GainSTMMode> for autd3::prelude::GainSTMMode {
    fn from(mode: GainSTMMode) -> Self {
        match mode {
            GainSTMMode::PhaseIntensityFull => autd3::prelude::GainSTMMode::PhaseIntensityFull,
            GainSTMMode::PhaseFull => autd3::prelude::GainSTMMode::PhaseFull,
            GainSTMMode::PhaseHalf => autd3::prelude::GainSTMMode::PhaseHalf,
        }
    }
}
