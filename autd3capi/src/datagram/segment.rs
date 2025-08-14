use autd3capi_driver::{
    DatagramPtr, TransitionModeWrap,
    autd3::core::firmware::Segment,
    driver::datagram::{
        SwapSegmentFociSTM, SwapSegmentGain, SwapSegmentGainSTM, SwapSegmentModulation,
    },
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentModulation(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegmentModulation(segment, transition_mode).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentFociSTM(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegmentFociSTM(segment, transition_mode).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGainSTM(
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    SwapSegmentGainSTM(segment, transition_mode).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGain(segment: Segment) -> DatagramPtr {
    SwapSegmentGain(segment).into()
}
