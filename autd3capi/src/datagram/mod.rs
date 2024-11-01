pub mod clear;
pub mod debug;
pub mod force_fan;
pub mod phase_corr;
pub mod pulse_width_encoder;
pub mod reads_fpga_state;
pub mod segment;
pub mod silencer;
pub mod stm;
pub mod synchronize;
pub mod with_parallel_threshold;
pub mod with_timeout;

use autd3capi_driver::{DatagramPtr, DynDatagramTuple};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramTuple(d1: DatagramPtr, d2: DatagramPtr) -> DatagramPtr {
    DynDatagramTuple {
        d1: d1.into(),
        d2: d2.into(),
    }
    .into()
}
