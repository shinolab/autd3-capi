use autd3capi_driver::{
    driver::{datagram::FocusSTM, defined::Hz, geometry::Vector3},
    *,
};
use driver::datagram::IntoDatagramWithSegmentTransition;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusFromFreq(
    freq: f64,
    points: *const f64,
    intensities: *const u8,
    size: u64,
) -> ResultFocusSTM {
    FocusSTM::from_freq(
        freq * Hz,
        (0..size as usize).map(|i| {
            let p = Vector3::new(
                points.add(i * 3).read(),
                points.add(i * 3 + 1).read(),
                points.add(i * 3 + 2).read(),
            );
            let intensity = *intensities.add(i);
            (p, intensity)
        }),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusFromFreqNearest(
    freq: f64,
    points: *const f64,
    intensities: *const u8,
    size: u64,
) -> ResultFocusSTM {
    FocusSTM::from_freq_nearest(
        freq * Hz,
        (0..size as usize).map(|i| {
            let p = Vector3::new(
                points.add(i * 3).read(),
                points.add(i * 3 + 1).read(),
                points.add(i * 3 + 2).read(),
            );
            let intensity = *intensities.add(i);
            (p, intensity)
        }),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusFromSamplingConfig(
    config: SamplingConfigWrap,
    points: *const f64,
    intensities: *const u8,
    size: u64,
) -> FocusSTMPtr {
    FocusSTM::from_sampling_config(
        config.into(),
        (0..size as usize).map(|i| {
            let p = Vector3::new(
                points.add(i * 3).read(),
                points.add(i * 3 + 1).read(),
                points.add(i * 3 + 2).read(),
            );
            let intensity = *intensities.add(i);
            (p, intensity)
        }),
    )
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
    take!(stm, FocusSTM)
        .with_segment(segment.into(), None)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusIntoDatagramWithSegmentTransition(
    stm: FocusSTMPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    take!(stm, FocusSTM)
        .with_segment(segment.into(), Some(transition_mode.into()))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusIntoDatagram(stm: FocusSTMPtr) -> DatagramPtr {
    (*take!(stm, FocusSTM)).into()
}
