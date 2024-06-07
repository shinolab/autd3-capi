use autd3::derive::Phase;
use autd3capi_driver::{
    driver::{datagram::FociSTM, defined::Hz, geometry::Vector3},
    *,
};
use driver::{
    datagram::IntoDatagramWithSegmentTransition,
    defined::{ControlPoint, ControlPoints},
};

unsafe fn control_points<const N: usize>(
    points: *const f32,
    offsets: *const u8,
    intensities: *const u8,
    size: u16,
) -> impl IntoIterator<Item = ControlPoints<N>> {
    (0..size as usize).map(move |i| {
        let intensity = *intensities.add(i);
        ControlPoints::from((
            core::array::from_fn::<usize, N, _>(std::convert::identity).map(|j| {
                let p = Vector3::new(
                    points.add((N * i + j) * 3).read(),
                    points.add((N * i + j) * 3 + 1).read(),
                    points.add((N * i + j) * 3 + 2).read(),
                );
                let offset = *offsets.add(N * i + j);
                ControlPoint::new(p).with_offset(Phase::new(offset))
            }),
            intensity,
        ))
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociFromFreq(
    freq: f32,
    points: *const f32,
    offsets: *const u8,
    intensities: *const u8,
    size: u16,
    n: u8,
) -> ResultFociSTM {
    match n {
        1 => FociSTM::from_freq(
            freq * Hz,
            control_points::<1>(points, offsets, intensities, size),
        )
        .into(),
        2 => FociSTM::from_freq(
            freq * Hz,
            control_points::<2>(points, offsets, intensities, size),
        )
        .into(),
        3 => FociSTM::from_freq(
            freq * Hz,
            control_points::<3>(points, offsets, intensities, size),
        )
        .into(),
        4 => FociSTM::from_freq(
            freq * Hz,
            control_points::<4>(points, offsets, intensities, size),
        )
        .into(),
        5 => FociSTM::from_freq(
            freq * Hz,
            control_points::<5>(points, offsets, intensities, size),
        )
        .into(),
        6 => FociSTM::from_freq(
            freq * Hz,
            control_points::<6>(points, offsets, intensities, size),
        )
        .into(),
        7 => FociSTM::from_freq(
            freq * Hz,
            control_points::<7>(points, offsets, intensities, size),
        )
        .into(),
        8 => FociSTM::from_freq(
            freq * Hz,
            control_points::<8>(points, offsets, intensities, size),
        )
        .into(),
        _ => unreachable!(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociFromFreqNearest(
    freq: f32,
    points: *const f32,
    offsets: *const u8,
    intensities: *const u8,
    size: u16,
    n: u8,
) -> ResultFociSTM {
    match n {
        1 => FociSTM::from_freq_nearest(
            freq * Hz,
            control_points::<1>(points, offsets, intensities, size),
        )
        .into(),
        2 => FociSTM::from_freq_nearest(
            freq * Hz,
            control_points::<2>(points, offsets, intensities, size),
        )
        .into(),
        3 => FociSTM::from_freq_nearest(
            freq * Hz,
            control_points::<3>(points, offsets, intensities, size),
        )
        .into(),
        4 => FociSTM::from_freq_nearest(
            freq * Hz,
            control_points::<4>(points, offsets, intensities, size),
        )
        .into(),
        5 => FociSTM::from_freq_nearest(
            freq * Hz,
            control_points::<5>(points, offsets, intensities, size),
        )
        .into(),
        6 => FociSTM::from_freq_nearest(
            freq * Hz,
            control_points::<6>(points, offsets, intensities, size),
        )
        .into(),
        7 => FociSTM::from_freq_nearest(
            freq * Hz,
            control_points::<7>(points, offsets, intensities, size),
        )
        .into(),
        8 => FociSTM::from_freq_nearest(
            freq * Hz,
            control_points::<8>(points, offsets, intensities, size),
        )
        .into(),
        _ => unreachable!(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociFromSamplingConfig(
    config: SamplingConfigWrap,
    points: *const f32,
    offsets: *const u8,
    intensities: *const u8,
    size: u16,
    n: u8,
) -> FociSTMPtr {
    match n {
        1 => FociSTM::from_sampling_config(
            config.into(),
            control_points::<1>(points, offsets, intensities, size),
        )
        .into(),
        2 => FociSTM::from_sampling_config(
            config.into(),
            control_points::<2>(points, offsets, intensities, size),
        )
        .into(),
        3 => FociSTM::from_sampling_config(
            config.into(),
            control_points::<3>(points, offsets, intensities, size),
        )
        .into(),
        4 => FociSTM::from_sampling_config(
            config.into(),
            control_points::<4>(points, offsets, intensities, size),
        )
        .into(),
        5 => FociSTM::from_sampling_config(
            config.into(),
            control_points::<5>(points, offsets, intensities, size),
        )
        .into(),
        6 => FociSTM::from_sampling_config(
            config.into(),
            control_points::<6>(points, offsets, intensities, size),
        )
        .into(),
        7 => FociSTM::from_sampling_config(
            config.into(),
            control_points::<7>(points, offsets, intensities, size),
        )
        .into(),
        8 => FociSTM::from_sampling_config(
            config.into(),
            control_points::<8>(points, offsets, intensities, size),
        )
        .into(),
        _ => unreachable!(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociWithLoopBehavior(
    stm: FociSTMPtr,
    n: u8,
    loop_behavior: LoopBehavior,
) -> FociSTMPtr {
    match n {
        1 => take!(stm, FociSTM<1>)
            .with_loop_behavior(loop_behavior.into())
            .into(),
        2 => take!(stm, FociSTM<2>)
            .with_loop_behavior(loop_behavior.into())
            .into(),
        3 => take!(stm, FociSTM<3>)
            .with_loop_behavior(loop_behavior.into())
            .into(),
        4 => take!(stm, FociSTM<4>)
            .with_loop_behavior(loop_behavior.into())
            .into(),
        5 => take!(stm, FociSTM<5>)
            .with_loop_behavior(loop_behavior.into())
            .into(),
        6 => take!(stm, FociSTM<6>)
            .with_loop_behavior(loop_behavior.into())
            .into(),
        7 => take!(stm, FociSTM<7>)
            .with_loop_behavior(loop_behavior.into())
            .into(),
        8 => take!(stm, FociSTM<8>)
            .with_loop_behavior(loop_behavior.into())
            .into(),
        _ => unreachable!(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithSegment(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
) -> DatagramPtr {
    match n {
        1 => take!(stm, FociSTM<1>)
            .with_segment(segment.into(), None)
            .into(),
        2 => take!(stm, FociSTM<2>)
            .with_segment(segment.into(), None)
            .into(),
        3 => take!(stm, FociSTM<3>)
            .with_segment(segment.into(), None)
            .into(),
        4 => take!(stm, FociSTM<4>)
            .with_segment(segment.into(), None)
            .into(),
        5 => take!(stm, FociSTM<5>)
            .with_segment(segment.into(), None)
            .into(),
        6 => take!(stm, FociSTM<6>)
            .with_segment(segment.into(), None)
            .into(),
        7 => take!(stm, FociSTM<7>)
            .with_segment(segment.into(), None)
            .into(),
        8 => take!(stm, FociSTM<8>)
            .with_segment(segment.into(), None)
            .into(),
        _ => unreachable!(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithSegmentTransition(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    match n {
        1 => take!(stm, FociSTM<1>)
            .with_segment(segment.into(), Some(transition_mode.into()))
            .into(),
        2 => take!(stm, FociSTM<2>)
            .with_segment(segment.into(), Some(transition_mode.into()))
            .into(),
        3 => take!(stm, FociSTM<3>)
            .with_segment(segment.into(), Some(transition_mode.into()))
            .into(),
        4 => take!(stm, FociSTM<4>)
            .with_segment(segment.into(), Some(transition_mode.into()))
            .into(),
        5 => take!(stm, FociSTM<5>)
            .with_segment(segment.into(), Some(transition_mode.into()))
            .into(),
        6 => take!(stm, FociSTM<6>)
            .with_segment(segment.into(), Some(transition_mode.into()))
            .into(),
        7 => take!(stm, FociSTM<7>)
            .with_segment(segment.into(), Some(transition_mode.into()))
            .into(),
        8 => take!(stm, FociSTM<8>)
            .with_segment(segment.into(), Some(transition_mode.into()))
            .into(),
        _ => unreachable!(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagram(stm: FociSTMPtr, n: u8) -> DatagramPtr {
    match n {
        1 => (*take!(stm, FociSTM<1>)).into(),
        2 => (*take!(stm, FociSTM<2>)).into(),
        3 => (*take!(stm, FociSTM<3>)).into(),
        4 => (*take!(stm, FociSTM<4>)).into(),
        5 => (*take!(stm, FociSTM<5>)).into(),
        6 => (*take!(stm, FociSTM<6>)).into(),
        7 => (*take!(stm, FociSTM<7>)).into(),
        8 => (*take!(stm, FociSTM<8>)).into(),
        _ => unreachable!(),
    }
}
