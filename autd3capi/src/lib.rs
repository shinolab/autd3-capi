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
    use autd3capi_driver::{
        autd3::controller::SpinSleeper, driver::geometry::Quaternion, AUTDStatus, Point3,
    };

    use super::*;

    #[test]
    fn simple() {
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

            let g = gain::focus::AUTDGainFocus(Point3::new(0., 0., 150.), Default::default());
            let m = modulation::r#static::AUTDModulationStatic(0xFF);

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
