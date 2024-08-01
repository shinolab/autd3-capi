#[repr(u8)]
pub enum SilencerTarget {
    Intensity = 0,
    PulseWidth = 1,
}

impl From<SilencerTarget> for autd3_driver::firmware::operation::SilencerTarget {
    fn from(mode: SilencerTarget) -> Self {
        match mode {
            SilencerTarget::Intensity => {
                autd3_driver::firmware::operation::SilencerTarget::Intensity
            }
            SilencerTarget::PulseWidth => {
                autd3_driver::firmware::operation::SilencerTarget::PulseWidth
            }
        }
    }
}

impl From<autd3_driver::firmware::operation::SilencerTarget> for SilencerTarget {
    fn from(mode: autd3_driver::firmware::operation::SilencerTarget) -> Self {
        match mode {
            autd3_driver::firmware::operation::SilencerTarget::Intensity => {
                SilencerTarget::Intensity
            }
            autd3_driver::firmware::operation::SilencerTarget::PulseWidth => {
                SilencerTarget::PulseWidth
            }
        }
    }
}
