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
pub unsafe extern "C" fn AUTDDatagramSwapSegmentFociSTM(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::FociSTM(segment.into(), transition_mode.into()).into()
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
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGain(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegment::Gain(segment.into(), transition_mode.into()).into()
}
