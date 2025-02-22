use autd3capi_driver::{
    autd3::core::{datagram::Segment, sampling_config::SamplingConfig},
    driver::datagram::{BoxedGain, GainSTM, GainSTMOption, WithLoopBehavior, WithSegment},
    *,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    config: SamplingConfigWrap,
    gains: *const GainPtr,
    size: u16,
    option: GainSTMOption,
) -> GainSTMPtr {
    GainSTM::<Vec<BoxedGain>, SamplingConfig> {
        gains: (0..size as usize)
            .map(|i| unsafe { *take!(gains.add(i).read(), BoxedGain) })
            .collect(),
        config: config.into(),
        option,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithSegment(
    stm: GainSTMPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    WithSegment {
        inner: unsafe { *take!(stm, GainSTM<Vec<BoxedGain>, SamplingConfig>) },
        segment,
        transition_mode: transition_mode.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithLoopBehavior(
    stm: GainSTMPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
    loop_behavior: LoopBehavior,
) -> DatagramPtr {
    WithLoopBehavior {
        inner: unsafe { *take!(stm, GainSTM<Vec<BoxedGain>, SamplingConfig>) },
        segment,
        transition_mode: transition_mode.into(),
        loop_behavior: loop_behavior.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagram(stm: GainSTMPtr) -> DatagramPtr {
    unsafe { *take!(stm, GainSTM<Vec<BoxedGain>, SamplingConfig>) }.into()
}
