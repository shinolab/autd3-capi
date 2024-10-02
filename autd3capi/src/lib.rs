#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    async_ffi::{FfiFuture, LocalFfiFuture},
    tokio::{self, runtime::Runtime},
    HandlePtr, ResultI32, RuntimePtr,
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
#[must_use]
pub unsafe extern "C" fn AUTDGetRuntimeHandle(runtime: RuntimePtr) -> HandlePtr {
    HandlePtr(runtime.handle() as *const _ as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteRuntime(runtime: RuntimePtr) {
    let _ = Box::from_raw(runtime.0 as *mut Runtime);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitResultI32(
    handle: HandlePtr,
    future: FfiFuture<ResultI32>,
) -> ResultI32 {
    handle.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitLocalResultI32(
    handle: HandlePtr,
    future: LocalFfiFuture<ResultI32>,
) -> ResultI32 {
    handle.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitResultController(
    handle: HandlePtr,
    future: FfiFuture<ResultController>,
) -> ResultController {
    handle.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitResultFPGAStateList(
    handle: HandlePtr,
    future: FfiFuture<ResultFPGAStateList>,
) -> ResultFPGAStateList {
    handle.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDWaitResultFirmwareVersionList(
    handle: HandlePtr,
    future: FfiFuture<ResultFirmwareVersionList>,
) -> ResultFirmwareVersionList {
    handle.block_on(future)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
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

            let handle = AUTDGetRuntimeHandle(runtime);

            let pos = [Vector3::new(0., 0., 0.)];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let builder = controller::builder::AUTDControllerBuilder(pos.as_ptr(), rot.as_ptr(), 1);
            let link_builder = link::nop::AUTDLinkNop();
            let cnt = controller::builder::AUTDControllerOpen(builder, link_builder, -1);
            let cnt = AUTDWaitResultController(handle, cnt);
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
            let result = AUTDWaitResultI32(handle, future);
            assert_eq!(AUTD3_TRUE, result.result);

            let future = controller::AUTDControllerClose(cnt);
            let result = AUTDWaitResultI32(handle, future);
            assert_eq!(AUTD3_TRUE, result.result);

            AUTDDeleteRuntime(runtime);
        }
    }
}
