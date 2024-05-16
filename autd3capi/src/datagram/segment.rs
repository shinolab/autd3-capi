use autd3capi_driver::{driver::datagram::SwapSegment, DatagramPtr, Segment, TransitionModeWrap};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentModulation(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::modulation(segment.into(), transition_mode.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentFocusSTM(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::focus_stm(segment.into(), transition_mode.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGainSTM(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::gain_stm(segment.into(), transition_mode.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGain(segment: Segment) -> DatagramPtr {
    SwapSegment::gain(segment.into()).into()
}
