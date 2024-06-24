#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    async_ffi::{FfiFuture, LocalFfiFuture},
    impl_ptr,
    tokio::{self, runtime::Runtime},
    trace_level_into, ResultI32,
};
use controller::{ResultController, ResultFPGAStateList, ResultFirmwareVersionList};

pub mod controller;
pub mod datagram;
pub mod driver;
pub mod gain;
pub mod geometry;
pub mod link;
pub mod modulation;
pub mod result;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RuntimePtr(pub *const libc::c_void);

unsafe impl Send for RuntimePtr {}
unsafe impl Sync for RuntimePtr {}

impl_ptr!(RuntimePtr, Runtime);

#[no_mangle]
pub unsafe extern "C" fn AUTDSetUltrasoundFreq(f: u32) {
    autd3capi_driver::driver::set_ultrasound_freq(f * autd3capi_driver::driver::defined::Hz);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCreateRuntime() -> RuntimePtr {
    RuntimePtr(Box::into_raw(Box::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap(),
    )) as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteRuntime(runtime: RuntimePtr) {
    let _ = Box::from_raw(runtime.0 as *mut Runtime);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitResultI32(
    runtime: RuntimePtr,
    future: FfiFuture<ResultI32>,
) -> ResultI32 {
    runtime.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitLocalResultI32(
    runtime: RuntimePtr,
    future: LocalFfiFuture<ResultI32>,
) -> ResultI32 {
    runtime.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitResultController(
    runtime: RuntimePtr,
    future: FfiFuture<ResultController>,
) -> ResultController {
    runtime.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitResultFPGAStateList(
    runtime: RuntimePtr,
    future: FfiFuture<ResultFPGAStateList>,
) -> ResultFPGAStateList {
    runtime.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitResultFirmwareVersionList(
    runtime: RuntimePtr,
    future: FfiFuture<ResultFirmwareVersionList>,
) -> ResultFirmwareVersionList {
    runtime.block_on(future)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTracingInit(level: u8) {
    tracing_subscriber::fmt()
        .with_max_level(trace_level_into(level))
        .init();
}

#[cfg(test)]
mod tests {
    use autd3capi_driver::{driver::geometry::Quaternion, Vector3, AUTD3_TRUE};

    use super::*;

    #[test]
    fn simple() {
        unsafe {
            let runtime = AUTDCreateRuntime();

            let pos = [Vector3::new(0., 0., 0.)];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let builder = controller::builder::AUTDControllerBuilder(pos.as_ptr(), rot.as_ptr(), 1);
            let link_builder = link::nop::AUTDLinkNop();
            let cnt = controller::builder::AUTDControllerOpen(builder, link_builder, -1);
            let cnt = AUTDWaitResultController(runtime, cnt);
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;

            let g = gain::focus::AUTDGainFocus(Vector3::new(0., 0., 150.), 0xFF, 0x00);
            let m = modulation::r#static::AUTDModulationStatic(
                0xFF,
                driver::firmware::fpga::loop_behavior::AUTDLoopBehaviorInfinite(),
            );

            let d1 = gain::AUTDGainIntoDatagram(g);
            let d2 = modulation::AUTDModulationIntoDatagram(m);
            let d = datagram::AUTDDatagramTuple(d1, d2);
            let future = controller::AUTDControllerSend(cnt, d);
            let result = AUTDWaitResultI32(runtime, future);
            assert_eq!(AUTD3_TRUE, result.result);

            let future = controller::AUTDControllerClose(cnt);
            let result = AUTDWaitResultI32(runtime, future);
            assert_eq!(AUTD3_TRUE, result.result);

            controller::AUTDControllerDelete(cnt);
            AUTDDeleteRuntime(runtime);
        }
    }
}
