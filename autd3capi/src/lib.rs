#![allow(clippy::missing_safety_doc)]

pub mod controller;
pub mod datagram;
pub mod driver;
pub mod environment;
pub mod gain;
pub mod geometry;
pub mod link;
pub mod modulation;
pub mod result;

#[cfg(test)]
mod tests {
    use autd3capi_driver::{AUTDStatus, Point3, autd3::driver::geometry::Quaternion};

    use super::*;

    #[test]
    fn simple() {
        unsafe {
            let pos = [Point3::origin()];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let option = controller::sender::SenderOption {
                send_interval: std::time::Duration::from_millis(1).into(),
                receive_interval: std::time::Duration::from_millis(1).into(),
                timeout: None.into(),
            };
            let sleeper = autd3capi_driver::SleeperTag::Std;
            let cnt = controller::AUTDControllerOpen(
                pos.as_ptr(),
                rot.as_ptr(),
                1,
                link::nop::AUTDLinkNop(),
                option,
                sleeper,
            );
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;
            let sender = controller::sender::AUTDSender(cnt, option, sleeper);

            let g = gain::focus::AUTDGainFocus(Point3::new(0., 0., 150.), Default::default());
            let m = modulation::r#static::AUTDModulationStatic(0xFF);

            let d1 = gain::AUTDGainIntoDatagram(g);
            let d2 = modulation::AUTDModulationIntoDatagram(m);
            let d = datagram::tuple::AUTDDatagramTuple(d1, d2);
            let result = controller::sender::AUTDSenderSend(sender, d);
            assert_eq!(AUTDStatus::AUTDTrue, result.result);

            let result = controller::AUTDControllerClose(cnt);
            assert_eq!(AUTDStatus::AUTDTrue, result.result);
        }
    }
}
