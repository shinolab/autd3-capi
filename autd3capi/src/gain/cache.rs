use autd3capi_driver::{take, take_gain, BoxedGainCache, GainPtr, G};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCache(g: GainPtr) -> GainPtr {
    BoxedGainCache::new(take!(g, Box<G>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCacheClone(g: GainPtr) -> GainPtr {
    (*(g.0 as *mut Box<G> as *mut Box<BoxedGainCache>)
        .as_ref()
        .unwrap()
        .clone())
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCacheFree(g: GainPtr) {
    let _ = take_gain!(g, BoxedGainCache);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use autd3capi_driver::{
        driver::geometry::Quaternion, ConstPtr, Drive, GeometryPtr, Vector3, AUTD3_TRUE,
    };

    use super::*;

    use crate::*;

    unsafe extern "C" fn f(
        context: ConstPtr,
        _: GeometryPtr,
        dev_idx: u16,
        tr_idx: u8,
        _: *mut Drive,
    ) {
        let i = context.0 as *mut i32;
        if dev_idx == 0 && tr_idx == 0 {
            unsafe {
                *i = *i + 1;
            }
        }
    }

    #[test]
    fn gain_cache() {
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

            let count = |gc: GainPtr| {
                Arc::strong_count(
                    &(gc.0 as *mut Box<G> as *mut Box<BoxedGainCache>)
                        .as_ref()
                        .unwrap()
                        .gain,
                )
            };

            let mut i: i32 = 0;
            let g = gain::custom::AUTDGainCustom(
                std::mem::transmute::<
                    unsafe extern "C" fn(ConstPtr, GeometryPtr, u16, u8, *mut Drive),
                    ConstPtr,
                >(f),
                ConstPtr(&mut i as *mut i32 as _),
                GeometryPtr(std::ptr::null_mut()),
            );
            let gc = AUTDGainCache(g);
            assert_eq!(1, count(gc));

            {
                let gg = AUTDGainCacheClone(gc);
                assert_eq!(2, count(gc));
                let d = gain::AUTDGainIntoDatagram(gg);
                let future = controller::AUTDControllerSend(cnt, d);
                let result = AUTDWaitResultI32(handle, future);
                assert_eq!(AUTD3_TRUE, result.result);
                assert_eq!(1, i);
            }
            assert_eq!(1, count(gc));

            {
                let gg = AUTDGainCacheClone(gc);
                assert_eq!(2, count(gc));
                let d = gain::AUTDGainIntoDatagram(gg);
                let future = controller::AUTDControllerSend(cnt, d);
                let result = AUTDWaitResultI32(handle, future);
                assert_eq!(AUTD3_TRUE, result.result);
                assert_eq!(1, i);
            }

            assert_eq!(1, count(gc));
            AUTDGainCacheFree(gc);

            let future = controller::AUTDControllerClose(cnt);
            let result = AUTDWaitResultI32(handle, future);
            assert_eq!(AUTD3_TRUE, result.result);

            AUTDDeleteRuntime(runtime);
        }
    }
}