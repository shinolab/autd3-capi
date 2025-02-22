#![allow(clippy::missing_safety_doc)]

pub mod constraint;
pub mod greedy;
pub mod gs;
pub mod gspat;
pub mod lm;
pub mod naive;
pub mod nalgebra_backend;

use autd3_gain_holo::*;
use autd3capi_driver::*;
use constraint::EmissionConstraintWrap;

#[repr(C)]
pub struct BackendPtr(pub *const libc::c_void);

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSPLToPascal(value: f32) -> f32 {
    (value * dB).pascal()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloPascalToSPL(value: f32) -> f32 {
    (value * Pa).spl()
}

#[macro_export]
macro_rules! create_holo {
    ($backend_type:tt, $direcivity:tt, $backend:expr, $points:expr, $amps:expr, $size:expr) => {{
        let points = vec_from_raw!($points, Point3, $size);
        let amps = vec_from_raw!($amps, f32, $size);
        let foci = points
            .into_iter()
            .zip(amps.into_iter())
            .map(|(p, a)| (p, a * Pa))
            .collect();
        let backend = ($backend.0 as *const std::sync::Arc<$backend_type<$direcivity>>)
            .as_ref()
            .unwrap()
            .clone();
        (foci, backend)
    }};

    ($direcivity:tt, $points:expr, $amps:expr, $size:expr) => {{
        let points = vec_from_raw!($points, Point3, $size);
        let amps = vec_from_raw!($amps, f32, $size);
        points
            .into_iter()
            .zip(amps.into_iter())
            .map(|(p, a)| (p, a * Pa))
            .collect()
    }};
}
