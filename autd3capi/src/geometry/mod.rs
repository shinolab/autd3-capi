#![allow(clippy::missing_safety_doc)]

pub mod device;
pub mod rotation;
pub mod transducer;

use autd3capi_driver::*;
use driver::{
    autd3_device::AUTD3,
    geometry::{Quaternion, UnitQuaternion},
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGeometry(cnt: ControllerPtr) -> GeometryPtr {
    GeometryPtr(cnt.geometry() as *const _ as _)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryNumDevices(geo: GeometryPtr) -> u32 {
    geo.num_devices() as _
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryNumTransducers(geo: GeometryPtr) -> u32 {
    geo.num_transducers() as _
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryCenter(geo: GeometryPtr) -> Point3 {
    geo.center()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDGeometryReconfigure(
    mut geo: GeometryPtr,
    pos: *const Point3,
    rot: *const Quaternion,
) {
    geo.reconfigure(|dev| {
        let pos = unsafe { pos.add(dev.idx()).read() };
        let rot = unsafe { rot.add(dev.idx()).read() };
        AUTD3 {
            pos,
            rot: UnitQuaternion::from_quaternion(rot),
        }
    });
}

#[cfg(test)]
mod tests {
    use autd3capi_driver::{
        Point3,
        autd3::{controller::ParallelMode, core::sleep::SpinSleeper, driver::geometry::Quaternion},
    };

    use crate::{controller, link};

    use super::*;

    #[test]
    fn geometry() {
        unsafe {
            let pos = [Point3::origin()];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let option = controller::sender::SenderOption {
                send_interval: std::time::Duration::from_millis(1).into(),
                receive_interval: std::time::Duration::from_millis(1).into(),
                timeout: None.into(),
                parallel: ParallelMode::Auto,
                strict: true,
            };
            let sleeper = autd3capi_driver::SleeperWrap {
                tag: autd3capi_driver::SleeperTag::Spin,
                value: SpinSleeper::default().native_accuracy_ns(),
                spin_strategy: SpinSleeper::default().spin_strategy().into(),
            };
            let timer_strategy = autd3capi_driver::TimerStrategyWrap {
                tag: autd3capi_driver::TimerStrategyTag::FixedSchedule,
                sleep: sleeper,
            };
            let cnt = controller::AUTDControllerOpen(
                pos.as_ptr(),
                rot.as_ptr(),
                1,
                link::nop::AUTDLinkNop(),
                option,
                timer_strategy,
            );
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;

            let geo = AUTDGeometry(cnt);
            assert_eq!(AUTDGeometryNumDevices(geo), 1);
            assert_eq!(AUTDGeometryNumTransducers(geo), 249);
        }
    }
}
