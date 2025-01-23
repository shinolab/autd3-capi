use autd3capi_driver::autd3::gain::GainCache;
use autd3capi_driver::{take, GainPtr};

use autd3capi_driver::driver::datagram::BoxedGain;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GainCachePtr(pub *const libc::c_void);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCache(g: GainPtr) -> GainCachePtr {
    GainCachePtr(Box::into_raw(Box::new(GainCache::new(*take!(g, BoxedGain)))) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCacheClone(g: GainCachePtr) -> GainPtr {
    (g.0 as *mut GainCache<BoxedGain>)
        .as_ref()
        .unwrap()
        .clone()
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCacheFree(g: GainCachePtr) {
    let _ = take!(g, GainCache<BoxedGain>);
}

#[cfg(test)]
mod tests {

    use autd3capi_driver::{
        autd3::{controller::SpinSleeper, core::gain::Drive},
        driver::geometry::Quaternion,
        AUTDStatus, ConstPtr, GeometryPtr, Point3,
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
                *i += 1;
            }
        }
    }

    #[test]
    fn gain_cache() {
        unsafe {
            let pos = [Point3::origin()];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let cnt = controller::AUTDControllerOpen(
                pos.as_ptr(),
                rot.as_ptr(),
                1,
                link::nop::AUTDLinkNop(),
                controller::sender::SenderOption {
                    send_interval: std::time::Duration::from_millis(1).into(),
                    receive_interval: std::time::Duration::from_millis(1).into(),
                    timeout: None.into(),
                    parallel_threshold: -1,
                    sleeper: autd3capi_driver::SleeperWrap {
                        tag: autd3capi_driver::SleeperTag::Spin,
                        value: SpinSleeper::default().native_accuracy_ns(),
                        spin_strategy: SpinSleeper::default().spin_strategy().into(),
                    },
                },
            );
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;

            let count = |gc: GainCachePtr| {
                (gc.0 as *mut GainCache<BoxedGain>)
                    .as_ref()
                    .unwrap()
                    .count()
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
                let result = controller::AUTDControllerSend(cnt, d);
                assert_eq!(AUTDStatus::AUTDTrue, result.result);
                assert_eq!(1, i);
            }
            assert_eq!(1, count(gc));

            {
                let gg = AUTDGainCacheClone(gc);
                assert_eq!(2, count(gc));
                let d = gain::AUTDGainIntoDatagram(gg);
                let result = controller::AUTDControllerSend(cnt, d);
                assert_eq!(AUTDStatus::AUTDTrue, result.result);
                assert_eq!(1, i);
            }
            assert_eq!(1, count(gc));
            AUTDGainCacheFree(gc);

            let result = controller::AUTDControllerClose(cnt);
            assert_eq!(AUTDStatus::AUTDTrue, result.result);
        }
    }
}
