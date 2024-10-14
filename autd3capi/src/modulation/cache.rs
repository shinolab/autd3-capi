use autd3capi_driver::{take, take_mod, BoxedModulationCache, LoopBehavior, ModulationPtr, M};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCache(m: ModulationPtr) -> ModulationPtr {
    BoxedModulationCache::new(take!(m, Box<M>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCacheClone(
    m: ModulationPtr,
    loop_behavior: LoopBehavior,
) -> ModulationPtr {
    (*(m.0 as *mut Box<M> as *mut Box<BoxedModulationCache>)
        .as_ref()
        .unwrap()
        .clone())
    .with_loop_behavior(loop_behavior.into())
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheFree(m: ModulationPtr) {
    let _ = take_mod!(m, BoxedModulationCache);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use autd3capi_driver::{driver::geometry::Quaternion, Vector3, AUTD3_TRUE};

    use super::*;

    use crate::*;

    #[test]
    fn modulation_cache() {
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

            let count = |mc: ModulationPtr| {
                Arc::strong_count(
                    &(mc.0 as *mut Box<M> as *mut Box<BoxedModulationCache>)
                        .as_ref()
                        .unwrap()
                        .m,
                )
            };

            let m = modulation::r#static::AUTDModulationStatic(
                0xFF,
                autd3capi_driver::autd3::driver::firmware::fpga::LoopBehavior::infinite().into(),
            );
            let mc = AUTDModulationCache(m);
            assert_eq!(1, count(mc));

            {
                let mm = AUTDModulationCacheClone(
                    mc,
                    autd3capi_driver::autd3::driver::firmware::fpga::LoopBehavior::infinite()
                        .into(),
                );
                assert_eq!(2, count(mc));
                let d = modulation::AUTDModulationIntoDatagram(mm);
                let future = controller::AUTDControllerSend(cnt, d);
                let result = AUTDWaitResultI32(handle, future);
                assert_eq!(AUTD3_TRUE, result.result);
            }
            assert_eq!(1, count(mc));

            {
                let mm = AUTDModulationCacheClone(
                    mc,
                    autd3capi_driver::autd3::driver::firmware::fpga::LoopBehavior::infinite()
                        .into(),
                );
                assert_eq!(2, count(mc));
                let d = modulation::AUTDModulationIntoDatagram(mm);
                let future = controller::AUTDControllerSend(cnt, d);
                let result = AUTDWaitResultI32(handle, future);
                assert_eq!(AUTD3_TRUE, result.result);
            }

            assert_eq!(1, count(mc));
            AUTDModulationCacheFree(mc);

            let future = controller::AUTDControllerClose(cnt);
            let result = AUTDWaitResultI32(handle, future);
            assert_eq!(AUTD3_TRUE, result.result);

            AUTDDeleteRuntime(runtime);
        }
    }
}
