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
use autd3capi_driver::*;
use constraint::EmissionConstraintWrap;

#[repr(C)]
pub struct BackendPtr(pub ConstPtr);

#[repr(C)]

pub struct ResultBackend {
    pub result: BackendPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSPLToPascal(value: f32) -> f32 {
    (value * dB).pascal()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloPascalToSPL(value: f32) -> f32 {
    (value * Pa).spl()
}

#[macro_export]
macro_rules! create_holo {
    ($type:tt, $backend_type:tt, $direcivity:tt, $backend:expr, $points:expr, $amps:expr, $size:expr) => {
        $type::<$direcivity, $backend_type<$direcivity>>::new(
            ($backend.0 as *const std::sync::Arc<$backend_type<$direcivity>>)
                .as_ref()
                .unwrap()
                .clone(),
            (0..$size as usize).map(|i| {
                let p = Vector3::new(
                    $points.add(i * 3).read(),
                    $points.add(i * 3 + 1).read(),
                    $points.add(i * 3 + 2).read(),
                );
                let amp = *$amps.add(i) * Pa;
                (p, amp)
            }),
        )
    };

    ($type:tt, $direcivity:tt, $points:expr, $amps:expr, $size:expr) => {
        $type::<$direcivity>::new((0..$size as usize).map(|i| {
            let p = Vector3::new(
                $points.add(i * 3).read(),
                $points.add(i * 3 + 1).read(),
                $points.add(i * 3 + 2).read(),
            );
            let amp = *$amps.add(i) * Pa;
            (p, amp)
        }))
    };
}
