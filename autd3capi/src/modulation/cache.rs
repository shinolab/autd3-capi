use autd3capi_driver::{
    autd3::datagram::modulation::ModulationCache,
    autd3::{derive::*, prelude::IntoModulationCache},
    driver::datagram::BoxedModulation,
    take, ModulationPtr,
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ModulationCachePtr(pub *const libc::c_void);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCache(m: ModulationPtr) -> ModulationCachePtr {
    ModulationCachePtr(Box::into_raw(Box::new((*take!(m, BoxedModulation)).with_cache())) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCacheClone(
    m: ModulationCachePtr,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    (m.0 as *mut ModulationCache<BoxedModulation>)
        .as_ref()
        .unwrap()
        .clone()
        .with_loop_behavior(loop_behavior)
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheFree(m: ModulationCachePtr) {
    let _ = take!(m, ModulationCache<BoxedModulation>);
}

#[cfg(test)]
mod tests {
    use autd3capi_driver::{driver::geometry::Quaternion, AUTDStatus, Vector3};
    use controller::timer::AUTDTimerStrategySpinDefault;

    use super::*;

    use crate::*;

    #[test]
    fn modulation_cache() {
        unsafe {
            let runtime = AUTDCreateRuntime();

            let handle = AUTDGetRuntimeHandle(runtime);

            let pos = [Vector3::new(0., 0., 0.)];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let builder = controller::builder::AUTDControllerBuilder(
                pos.as_ptr(),
                rot.as_ptr(),
                1,
                4,
                20_000_000,
                1_000_000,
                1_000_000,
                AUTDTimerStrategySpinDefault(),
            );
            let link_builder = link::nop::AUTDLinkNop();
            let cnt = controller::builder::AUTDControllerOpen(builder, link_builder, -1);
            let cnt = AUTDWaitResultController(handle, cnt);
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;

            let count = |gc: ModulationCachePtr| {
                (gc.0 as *mut ModulationCache<BoxedModulation>)
                    .as_ref()
                    .unwrap()
                    .count()
            };

            let m = modulation::r#static::AUTDModulationStatic(
                0xFF,
                autd3capi_driver::autd3::driver::firmware::fpga::LoopBehavior::infinite(),
            );
            let mc = AUTDModulationCache(m);
            assert_eq!(1, count(mc));

            {
                let mm = AUTDModulationCacheClone(
                    mc,
                    autd3capi_driver::autd3::driver::firmware::fpga::LoopBehavior::infinite(),
                );
                assert_eq!(2, count(mc));
                let d = modulation::AUTDModulationIntoDatagram(mm);
                let future = controller::AUTDControllerSend(cnt, d);
                let result = AUTDWaitResultStatus(handle, future);
                assert_eq!(AUTDStatus::TRUE, result.result);
            }
            assert_eq!(1, count(mc));

            {
                let mm = AUTDModulationCacheClone(
                    mc,
                    autd3capi_driver::autd3::driver::firmware::fpga::LoopBehavior::infinite(),
                );
                assert_eq!(2, count(mc));
                let d = modulation::AUTDModulationIntoDatagram(mm);
                let future = controller::AUTDControllerSend(cnt, d);
                let result = AUTDWaitResultStatus(handle, future);
                assert_eq!(AUTDStatus::TRUE, result.result);
            }

            assert_eq!(1, count(mc));
            AUTDModulationCacheFree(mc);

            let future = controller::AUTDControllerClose(cnt);
            let result = AUTDWaitResultStatus(handle, future);
            assert_eq!(AUTDStatus::TRUE, result.result);

            AUTDDeleteRuntime(runtime);
        }
    }
}
