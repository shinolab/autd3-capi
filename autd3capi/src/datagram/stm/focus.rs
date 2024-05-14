use autd3capi_driver::{
    driver::{
        datagram::{FocusSTM, SwapSegment},
        defined::Hz,
        geometry::Vector3,
    },
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusFromFreq(freq: f64) -> FocusSTMPtr {
    FocusSTM::from_freq(freq * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusFromFreqNearest(freq: f64) -> FocusSTMPtr {
    FocusSTM::from_freq_nearest(freq * Hz).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusFromSamplingConfig(config: SamplingConfigPtr) -> FocusSTMPtr {
    FocusSTM::from_sampling_config(*take!(config, _)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusAddFoci(
    stm: FocusSTMPtr,
    points: *const f64,
    intensities: *const u8,
    size: u64,
) -> FocusSTMPtr {
    take!(stm, FocusSTM)
        .add_foci_from_iter((0..size as usize).map(|i| {
            let p = Vector3::new(
                points.add(i * 3).read(),
                points.add(i * 3 + 1).read(),
                points.add(i * 3 + 2).read(),
            );
            let intensity = *intensities.add(i);
            (p, intensity)
        }))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusWithLoopBehavior(
    stm: FocusSTMPtr,
    loop_behavior: LoopBehavior,
) -> FocusSTMPtr {
    take!(stm, FocusSTM)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusIntoDatagramWithSegment(
    stm: FocusSTMPtr,
    segment: Segment,
) -> DatagramPtr {
    take!(stm, FocusSTM).with_segment(segment, None).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusIntoDatagramWithSegmentTransition(
    stm: FocusSTMPtr,
    segment: Segment,
    transition_mode: TransitionMode,
) -> DatagramPtr {
    take!(stm, FocusSTM)
        .with_segment(segment, Some(transition_mode))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusIntoDatagram(stm: FocusSTMPtr) -> DatagramPtr {
    (*take!(stm, FocusSTM)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentFocusSTM(
    segment: Segment,
    transition_mode: TransitionMode,
) -> DatagramPtr {
    SwapSegment::focus_stm(segment.into(), transition_mode.into()).into()
}
