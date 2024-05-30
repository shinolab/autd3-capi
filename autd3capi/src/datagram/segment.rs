use autd3capi_driver::{driver::datagram::SwapSegment, DatagramPtr, Segment, TransitionModeWrap};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentModulation(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::Modulation(segment.into(), transition_mode.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentFocusSTM(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::FocusSTM(segment.into(), transition_mode.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGainSTM(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::GainSTM(segment.into(), transition_mode.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGain(segment: Segment) -> DatagramPtr {
    SwapSegment::Gain(segment.into()).into()
}
