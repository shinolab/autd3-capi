use autd3_driver::firmware::fpga::SilencerTarget as NativeSilencerTarget;

#[repr(u8)]
pub enum SilencerTarget {
    Intensity = 0,
    PulseWidth = 1,
}

impl From<SilencerTarget> for NativeSilencerTarget {
    fn from(mode: SilencerTarget) -> Self {
        match mode {
            SilencerTarget::Intensity => NativeSilencerTarget::Intensity,
            SilencerTarget::PulseWidth => NativeSilencerTarget::PulseWidth,
        }
    }
}

impl From<NativeSilencerTarget> for SilencerTarget {
    fn from(mode: NativeSilencerTarget) -> Self {
        match mode {
            NativeSilencerTarget::Intensity => SilencerTarget::Intensity,
            NativeSilencerTarget::PulseWidth => SilencerTarget::PulseWidth,
        }
    }
}
