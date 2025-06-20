use autd3::core::{datagram::Segment, modulation::Modulation};
use autd3capi_driver::*;
use driver::datagram::{BoxedModulation, WithLoopBehavior, WithSegment};

pub mod custom;
pub mod fir;
pub mod fourier;
pub mod radiation_pressure;
pub mod sine;
pub mod square;
pub mod r#static;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingConfig(m: ModulationPtr) -> SamplingConfigWrap {
    unsafe { take!(m, BoxedModulation) }
        .sampling_config()
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithSegment(
    m: ModulationPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    WithSegment {
        inner: unsafe { *take!(m, BoxedModulation) },
        segment,
        transition_mode: transition_mode.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithLoopBehavior(
    m: ModulationPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
    loop_behavior: LoopBehavior,
) -> DatagramPtr {
    WithLoopBehavior {
        inner: unsafe { *take!(m, BoxedModulation) },
        segment,
        transition_mode: transition_mode.into(),
        loop_behavior: loop_behavior.into(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagram(m: ModulationPtr) -> DatagramPtr {
    unsafe { *take!(m, BoxedModulation) }.into()
}
