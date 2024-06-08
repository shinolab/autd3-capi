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
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => FociSTM::from_freq(
                    freq * Hz,
                    control_points::<N>(points, offsets, intensities, size),
                )
                .into(),)*
            _ => unreachable!(),
        }
    })
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
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => FociSTM::from_freq_nearest(
                    freq * Hz,
                    control_points::<N>(points, offsets, intensities, size),
                )
                .into(),)*
            _ => unreachable!(),
        }
    })
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
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => FociSTM::from_sampling_config(
                    config.into(),
                    control_points::<N>(points, offsets, intensities, size),
                )
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociWithLoopBehavior(
    stm: FociSTMPtr,
    n: u8,
    loop_behavior: LoopBehavior,
) -> FociSTMPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => take!(stm, FociSTM<N>)
                .with_loop_behavior(loop_behavior.into())
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithSegment(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => take!(stm, FociSTM<N>)
                .with_segment(segment.into(), None)
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithSegmentTransition(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => take!(stm, FociSTM<N>)
                .with_segment(segment.into(), Some(transition_mode.into()))
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagram(stm: FociSTMPtr, n: u8) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => (*take!(stm, FociSTM<1>)).into(),)*
            _ => unreachable!(),
        }
    })
}
