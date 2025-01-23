use autd3::core::{datagram::Segment, modulation::Modulation};
use autd3capi_driver::*;
use driver::datagram::{BoxedModulation, WithLoopBehavior, WithSegment};

pub mod cache;
pub mod custom;
pub mod fir;
pub mod fourier;
pub mod radiation_pressure;
pub mod sine;
pub mod square;
pub mod r#static;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingConfig(m: ModulationPtr) -> ResultSamplingConfig {
    (m.0 as *const BoxedModulation)
        .as_ref()
        .unwrap()
        .sampling_config()
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithSegment(
    m: ModulationPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    WithSegment {
        inner: (*take!(m, BoxedModulation)),
        segment,
        transition_mode: transition_mode.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithLoopBehavior(
    m: ModulationPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
    loop_behavior: LoopBehavior,
) -> DatagramPtr {
    WithLoopBehavior {
        inner: (*take!(m, BoxedModulation)),
        segment,
        transition_mode: transition_mode.into(),
        loop_behavior: loop_behavior.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagram(m: ModulationPtr) -> DatagramPtr {
    (*take!(m, BoxedModulation)).into()
}
