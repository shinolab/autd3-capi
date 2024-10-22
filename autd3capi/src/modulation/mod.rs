use autd3::derive::{ModulationProperty, SamplingConfig};
use autd3capi_driver::*;
use driver::datagram::IntoDatagramWithSegment;

pub mod cache;
pub mod custom;
pub mod fir;
// pub mod fourier;
pub mod radiation_pressure;
// pub mod sine;
// pub mod square;
pub mod r#static;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingConfig(m: ModulationPtr) -> SamplingConfig {
    (m.0 as *const BoxedModulation)
        .as_ref()
        .unwrap()
        .sampling_config()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithSegment(
    m: ModulationPtr,
    segment: Segment,
) -> DatagramPtr {
    (*take!(m, BoxedModulation))
        .with_segment(segment.into(), None)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithSegmentTransition(
    m: ModulationPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    (*take!(m, BoxedModulation))
        .with_segment(segment.into(), Some(transition_mode.into()))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagram(m: ModulationPtr) -> DatagramPtr {
    (*take!(m, BoxedModulation)).into()
}
