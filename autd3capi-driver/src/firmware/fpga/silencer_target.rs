#[repr(u8)]
pub enum SilencerTarget {
    Intensity = 0,
    PulseWidth = 1,
}

impl From<SilencerTarget> for autd3::prelude::SilencerTarget {
    fn from(mode: SilencerTarget) -> Self {
        match mode {
            SilencerTarget::Intensity => autd3::prelude::SilencerTarget::Intensity,
            SilencerTarget::PulseWidth => autd3::prelude::SilencerTarget::PulseWidth,
        }
    }
}
