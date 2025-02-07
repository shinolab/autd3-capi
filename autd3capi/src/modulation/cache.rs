use autd3capi_driver::{
    autd3::datagram::modulation::Cache, driver::datagram::BoxedModulation, take, ModulationPtr,
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ModulationCachePtr(pub *const libc::c_void);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCache(m: ModulationPtr) -> ModulationCachePtr {
    ModulationCachePtr(Box::into_raw(Box::new(Cache::new(*take!(m, BoxedModulation)))) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCacheClone(m: ModulationCachePtr) -> ModulationPtr {
    (m.0 as *mut Cache<BoxedModulation>)
        .as_ref()
        .unwrap()
        .clone()
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheFree(m: ModulationCachePtr) {
    let _ = take!(m, Cache<BoxedModulation>);
}

#[cfg(test)]
mod tests {
    use autd3capi_driver::{
        autd3::controller::{ParallelMode, SpinSleeper},
        driver::geometry::Quaternion,
        AUTDStatus, Point3,
    };

    use super::*;

    use crate::*;

    #[test]
    fn modulation_cache() {
        unsafe {
            let pos = [Point3::origin()];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let option = controller::sender::SenderOption {
                send_interval: std::time::Duration::from_millis(1).into(),
                receive_interval: std::time::Duration::from_millis(1).into(),
                timeout: None.into(),
                parallel: ParallelMode::Auto,
                sleeper: autd3capi_driver::SleeperWrap {
                    tag: autd3capi_driver::SleeperTag::Spin,
                    value: SpinSleeper::default().native_accuracy_ns(),
                    spin_strategy: SpinSleeper::default().spin_strategy().into(),
                },
            };
            let cnt = controller::AUTDControllerOpen(
                pos.as_ptr(),
                rot.as_ptr(),
                1,
                link::nop::AUTDLinkNop(),
                option,
            );
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;

            let count = |gc: ModulationCachePtr| {
                (gc.0 as *mut Cache<BoxedModulation>)
                    .as_ref()
                    .unwrap()
                    .count()
            };

            let m = modulation::r#static::AUTDModulationStatic(0xFF);
            let mc = AUTDModulationCache(m);
            assert_eq!(1, count(mc));

            let sender = controller::sender::AUTDSender(cnt, option);
            {
                let mm = AUTDModulationCacheClone(mc);
                assert_eq!(2, count(mc));
                let d = modulation::AUTDModulationIntoDatagram(mm);
                let result = controller::sender::AUTDSenderSend(sender, d);
                assert_eq!(AUTDStatus::AUTDTrue, result.result);
            }
            assert_eq!(1, count(mc));

            {
                let mm = AUTDModulationCacheClone(mc);
                assert_eq!(2, count(mc));
                let d = modulation::AUTDModulationIntoDatagram(mm);
                let result = controller::sender::AUTDSenderSend(sender, d);
                assert_eq!(AUTDStatus::AUTDTrue, result.result);
            }

            assert_eq!(1, count(mc));
            AUTDModulationCacheFree(mc);

            let result = controller::AUTDControllerClose(cnt);
            assert_eq!(AUTDStatus::AUTDTrue, result.result);
        }
    }
}
