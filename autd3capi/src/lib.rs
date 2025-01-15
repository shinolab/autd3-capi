#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{validate_cstr, AUTDStatus, ConstPtr, ResultStatus};
use libc::c_char;

pub mod controller;
pub mod datagram;
pub mod driver;
pub mod gain;
pub mod geometry;
pub mod link;
pub mod modulation;
pub mod result;

#[no_mangle]
pub unsafe extern "C" fn AUTDTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTracingInitWithFile(path: *const c_char) -> ResultStatus {
    let path = validate_cstr!(path, AUTDStatus, ResultStatus);
    std::fs::File::options()
        .append(true)
        .create(true)
        .open(path)
        .map(|f| {
            tracing_subscriber::fmt()
                .with_writer(f)
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_ansi(false)
                .init();
            AUTDStatus::AUTDTrue
        })
        .into()
}

#[cfg(test)]
mod tests {
    use autd3capi_driver::{driver::geometry::Quaternion, AUTDStatus, OptionDuration, Point3};

    use super::*;

    #[test]
    fn simple() {
        unsafe {
            let pos = [Point3::origin()];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let builder = controller::builder::AUTDControllerBuilder(
                pos.as_ptr(),
                rot.as_ptr(),
                1,
                4,
                std::time::Duration::from_millis(20).into(),
                std::time::Duration::from_millis(1).into(),
                std::time::Duration::from_millis(1).into(),
                controller::timer::AUTDTimerStrategySpin(
                    controller::timer::AUTDTimerStrategySpinDefaultAccuracy(),
                    autd3capi_driver::SpinStrategyTag::SpinLoopHint,
                ),
            );
            let link_builder = link::nop::AUTDLinkNop();
            let cnt = controller::builder::AUTDControllerOpen(
                builder,
                link_builder,
                OptionDuration::NONE,
            );
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;

            let g = gain::focus::AUTDGainFocus(Point3::new(0., 0., 150.), 0xFF, 0x00);
            let m = modulation::r#static::AUTDModulationStatic(
                0xFF,
                driver::firmware::fpga::loop_behavior::AUTDLoopBehaviorInfinite(),
            );

            let d1 = gain::AUTDGainIntoDatagram(g);
            let d2 = modulation::AUTDModulationIntoDatagram(m);
            let d = datagram::AUTDDatagramTuple(d1, d2);
            let result = controller::AUTDControllerSend(cnt, d);
            assert_eq!(AUTDStatus::AUTDTrue, result.result);

            let result = controller::AUTDControllerClose(cnt);
            assert_eq!(AUTDStatus::AUTDTrue, result.result);
        }
    }
}
