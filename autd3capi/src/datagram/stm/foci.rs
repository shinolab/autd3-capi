use std::num::NonZeroU16;

use autd3capi_driver::{
    core::firmware::{SamplingConfig, Segment},
    driver::datagram::{ControlPoints, FociSTM, WithFiniteLoop, WithSegment},
    *,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFoci(
    config: SamplingConfigWrap,
    points: ConstPtr,
    size: u16,
    n: u8,
) -> FociSTMPtr {
    unsafe {
        match n {
            1 => {
                let points = points.0 as *const ControlPoints<1>;
                FociSTM::<1, Vec<_>, SamplingConfig> {
                    foci: (0..size as usize).map(|i| points.add(i).read()).collect(),
                    config: config.into(),
                }
                .into()
            }
            2 => {
                let points = points.0 as *const ControlPoints<2>;
                FociSTM::<2, Vec<_>, SamplingConfig> {
                    foci: (0..size as usize).map(|i| points.add(i).read()).collect(),
                    config: config.into(),
                }
                .into()
            }
            3 => {
                let points = points.0 as *const ControlPoints<3>;
                FociSTM::<3, Vec<_>, SamplingConfig> {
                    foci: (0..size as usize).map(|i| points.add(i).read()).collect(),
                    config: config.into(),
                }
                .into()
            }
            4 => {
                let points = points.0 as *const ControlPoints<4>;
                FociSTM::<4, Vec<_>, SamplingConfig> {
                    foci: (0..size as usize).map(|i| points.add(i).read()).collect(),
                    config: config.into(),
                }
                .into()
            }
            5 => {
                let points = points.0 as *const ControlPoints<5>;
                FociSTM::<5, Vec<_>, SamplingConfig> {
                    foci: (0..size as usize).map(|i| points.add(i).read()).collect(),
                    config: config.into(),
                }
                .into()
            }
            6 => {
                let points = points.0 as *const ControlPoints<6>;
                FociSTM::<6, Vec<_>, SamplingConfig> {
                    foci: (0..size as usize).map(|i| points.add(i).read()).collect(),
                    config: config.into(),
                }
                .into()
            }
            7 => {
                let points = points.0 as *const ControlPoints<7>;
                FociSTM::<7, Vec<_>, SamplingConfig> {
                    foci: (0..size as usize).map(|i| points.add(i).read()).collect(),
                    config: config.into(),
                }
                .into()
            }
            8 => {
                let points = points.0 as *const ControlPoints<8>;
                FociSTM::<8, Vec<_>, SamplingConfig> {
                    foci: (0..size as usize).map(|i| points.add(i).read()).collect(),
                    config: config.into(),
                }
                .into()
            }
            _ => unreachable!(),
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithSegment(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    match n {
        1 => WithSegment {
            inner: unsafe { *take!(stm, FociSTM<1, Vec<ControlPoints<1>>, SamplingConfig>) },
            segment,
            transition_mode,
        }
        .into(),
        2 => WithSegment {
            inner: unsafe { *take!(stm, FociSTM<2, Vec<ControlPoints<2>>, SamplingConfig>) },
            segment,
            transition_mode,
        }
        .into(),
        3 => WithSegment {
            inner: unsafe { *take!(stm, FociSTM<3, Vec<ControlPoints<3>>, SamplingConfig>) },
            segment,
            transition_mode,
        }
        .into(),
        4 => WithSegment {
            inner: unsafe { *take!(stm, FociSTM<4, Vec<ControlPoints<4>>, SamplingConfig>) },
            segment,
            transition_mode,
        }
        .into(),
        5 => WithSegment {
            inner: unsafe { *take!(stm, FociSTM<5, Vec<ControlPoints<5>>, SamplingConfig>) },
            segment,
            transition_mode,
        }
        .into(),
        6 => WithSegment {
            inner: unsafe { *take!(stm, FociSTM<6, Vec<ControlPoints<6>>, SamplingConfig>) },
            segment,
            transition_mode,
        }
        .into(),
        7 => WithSegment {
            inner: unsafe { *take!(stm, FociSTM<7, Vec<ControlPoints<7>>, SamplingConfig>) },
            segment,
            transition_mode,
        }
        .into(),
        8 => WithSegment {
            inner: unsafe { *take!(stm, FociSTM<8, Vec<ControlPoints<8>>, SamplingConfig>) },
            segment,
            transition_mode,
        }
        .into(),
        _ => unreachable!(),
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithFiniteLoop(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
    transition_mode: TransitionModeWrap,
    loop_count: u16,
) -> DatagramPtr {
    match n {
        1 => WithFiniteLoop {
            inner: unsafe { *take!(stm, FociSTM<1, Vec<ControlPoints<1>>, SamplingConfig>) },
            segment,
            transition_mode,
            loop_count: NonZeroU16::new(loop_count).unwrap(),
        }
        .into(),
        2 => WithFiniteLoop {
            inner: unsafe { *take!(stm, FociSTM<2, Vec<ControlPoints<2>>, SamplingConfig>) },
            segment,
            transition_mode,
            loop_count: NonZeroU16::new(loop_count).unwrap(),
        }
        .into(),
        3 => WithFiniteLoop {
            inner: unsafe { *take!(stm, FociSTM<3, Vec<ControlPoints<3>>, SamplingConfig>) },
            segment,
            transition_mode,
            loop_count: NonZeroU16::new(loop_count).unwrap(),
        }
        .into(),
        4 => WithFiniteLoop {
            inner: unsafe { *take!(stm, FociSTM<4, Vec<ControlPoints<4>>, SamplingConfig>) },
            segment,
            transition_mode,
            loop_count: NonZeroU16::new(loop_count).unwrap(),
        }
        .into(),
        5 => WithFiniteLoop {
            inner: unsafe { *take!(stm, FociSTM<5, Vec<ControlPoints<5>>, SamplingConfig>) },
            segment,
            transition_mode,
            loop_count: NonZeroU16::new(loop_count).unwrap(),
        }
        .into(),
        6 => WithFiniteLoop {
            inner: unsafe { *take!(stm, FociSTM<6, Vec<ControlPoints<6>>, SamplingConfig>) },
            segment,
            transition_mode,
            loop_count: NonZeroU16::new(loop_count).unwrap(),
        }
        .into(),
        7 => WithFiniteLoop {
            inner: unsafe { *take!(stm, FociSTM<7, Vec<ControlPoints<7>>, SamplingConfig>) },
            segment,
            transition_mode,
            loop_count: NonZeroU16::new(loop_count).unwrap(),
        }
        .into(),
        8 => WithFiniteLoop {
            inner: unsafe { *take!(stm, FociSTM<8, Vec<ControlPoints<8>>, SamplingConfig>) },
            segment,
            transition_mode,
            loop_count: NonZeroU16::new(loop_count).unwrap(),
        }
        .into(),
        _ => unreachable!(),
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagram(stm: FociSTMPtr, n: u8) -> DatagramPtr {
    match n {
        1 => unsafe { *take!(stm, FociSTM<1, Vec<ControlPoints<1>>, SamplingConfig>) }.into(),
        2 => unsafe { *take!(stm, FociSTM<2, Vec<ControlPoints<2>>, SamplingConfig>) }.into(),
        3 => unsafe { *take!(stm, FociSTM<3, Vec<ControlPoints<3>>, SamplingConfig>) }.into(),
        4 => unsafe { *take!(stm, FociSTM<4, Vec<ControlPoints<4>>, SamplingConfig>) }.into(),
        5 => unsafe { *take!(stm, FociSTM<5, Vec<ControlPoints<5>>, SamplingConfig>) }.into(),
        6 => unsafe { *take!(stm, FociSTM<6, Vec<ControlPoints<6>>, SamplingConfig>) }.into(),
        7 => unsafe { *take!(stm, FociSTM<7, Vec<ControlPoints<7>>, SamplingConfig>) }.into(),
        8 => unsafe { *take!(stm, FociSTM<8, Vec<ControlPoints<8>>, SamplingConfig>) }.into(),
        _ => unreachable!(),
    }
}
