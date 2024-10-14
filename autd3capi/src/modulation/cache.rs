use std::sync::Mutex;

use autd3capi_driver::{autd3::derive::*, take, take_mod, ModulationPtr, M};

use derive_more::{Debug, Deref};

#[derive(Modulation, Clone, Deref, Debug)]
pub struct BoxedCache {
    #[deref]
    m: Arc<M>,
    #[debug("{}", !self.cache.lock().unwrap().is_empty())]
    cache: Arc<Mutex<Arc<Vec<u8>>>>,
    #[no_change]
    config: SamplingConfig,
    loop_behavior: LoopBehavior,
}

impl BoxedCache {
    fn new(m: Box<M>) -> Self {
        Self {
            config: m.sampling_config(),
            loop_behavior: m.loop_behavior(),
            m: Arc::from(m),
            cache: Default::default(),
        }
    }

    pub fn init(&self) -> Result<(), AUTDInternalError> {
        if self.cache.lock().unwrap().is_empty() {
            tracing::debug!("Initialize cache");
            *self.cache.lock().unwrap() = self.m.calc()?;
        }
        Ok(())
    }
}

impl Modulation for BoxedCache {
    fn calc(&self) -> Result<Arc<Vec<u8>>, AUTDInternalError> {
        self.init()?;
        let buffer = self.cache.lock().unwrap().clone();
        Ok(buffer)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCache(m: ModulationPtr) -> ModulationPtr {
    BoxedCache::new(take!(m, Box<M>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCacheClone(m: ModulationPtr) -> ModulationPtr {
    (*(m.0 as *mut Box<M> as *mut Box<BoxedCache>)
        .as_ref()
        .unwrap()
        .clone())
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheFree(m: ModulationPtr) {
    let _ = take_mod!(m, BoxedCache);
}

#[cfg(test)]
mod tests {
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
                    &(mc.0 as *mut Box<M> as *mut Box<BoxedCache>)
                        .as_ref()
                        .unwrap()
                        .m,
                )
            };

            let m =
                modulation::r#static::AUTDModulationStatic(0xFF, LoopBehavior::infinite().into());
            let mc = AUTDModulationCache(m);
            assert_eq!(1, count(mc));

            {
                let mm = AUTDModulationCacheClone(mc);
                assert_eq!(2, count(mc));
                let d = modulation::AUTDModulationIntoDatagram(mm);
                let future = controller::AUTDControllerSend(cnt, d);
                let result = AUTDWaitResultI32(handle, future);
                assert_eq!(AUTD3_TRUE, result.result);
            }
            assert_eq!(1, count(mc));

            {
                let mm = AUTDModulationCacheClone(mc);
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
