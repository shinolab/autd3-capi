#![allow(clippy::missing_safety_doc)]

pub mod constraint;
pub mod greedy;
pub mod gs;
pub mod gspat;
pub mod lm;
pub mod naive;
pub mod nalgebra_backend;
pub mod sdp;

use autd3_gain_holo::*;
use autd3capi_def::*;
use constraint::EmissionConstraintPtr;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BackendPtr(pub ConstPtr);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultBackend {
    pub result: BackendPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSPLToPascal(value: float) -> float {
    (value * dB).as_pascal()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloPascalToSPL(value: float) -> float {
    (value * Pascal).as_spl()
}

#[macro_export]
macro_rules! create_holo {
    ($type:tt, $backend_type:tt, $backend:expr, $points:expr, $amps:expr, $size:expr) => {
        $type::new(
            ($backend.0 as *const std::sync::Arc<$backend_type>)
                .as_ref()
                .unwrap()
                .clone(),
        )
        .add_foci_from_iter((0..$size as usize).map(|i| {
            let p = Vector3::new(
                $points.add(i * 3).read(),
                $points.add(i * 3 + 1).read(),
                $points.add(i * 3 + 2).read(),
            );
            let amp = *$amps.add(i) * Pascal;
            (p, amp)
        }))
    };

    ($type:tt, $points:expr, $amps:expr, $size:expr) => {
        $type::new().add_foci_from_iter((0..$size as usize).map(|i| {
            let p = Vector3::new(
                $points.add(i * 3).read(),
                $points.add(i * 3 + 1).read(),
                $points.add(i * 3 + 2).read(),
            );
            let amp = *$amps.add(i) * Pascal;
            (p, amp)
        }))
    };
}
