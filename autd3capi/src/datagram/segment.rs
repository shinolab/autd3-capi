use autd3capi_driver::{
    DatagramPtr, TransitionModeWrap,
    autd3::core::datagram::{Segment, TransitionMode},
    driver::datagram::SwapSegment,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentModulation(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::Modulation(
        segment,
        Option::<TransitionMode>::from(transition_mode).unwrap(),
    )
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentFociSTM(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::FociSTM(
        segment,
        Option::<TransitionMode>::from(transition_mode).unwrap(),
    )
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGainSTM(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::GainSTM(
        segment,
        Option::<TransitionMode>::from(transition_mode).unwrap(),
    )
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGain(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::Gain(
        segment,
        Option::<TransitionMode>::from(transition_mode).unwrap(),
    )
    .into()
}
