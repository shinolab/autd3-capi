#![allow(clippy::missing_safety_doc)]

pub mod constraint;
pub mod greedy;
pub mod gs;
pub mod gspat;
pub mod naive;

use autd3_gain_holo::*;
use constraint::EmissionConstraintWrap;

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
    ($points:expr, $amps:expr, $size:expr) => {{
        let points = vec_from_raw!($points, Point3, $size);
        let amps = vec_from_raw!($amps, f32, $size);
        points
            .into_iter()
            .zip(amps.into_iter())
            .map(|(p, a)| (p, a * Pa))
            .collect()
    }};
}
