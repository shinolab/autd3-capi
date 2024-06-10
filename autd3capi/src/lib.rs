#![allow(clippy::missing_safety_doc)]

pub mod controller;
pub mod datagram;
pub mod driver;
pub mod gain;
pub mod geometry;
pub mod link;
pub mod modulation;
pub mod result;

#[cfg(test)]
mod tests {
    use autd3capi_driver::{driver::geometry::Quaternion, Vector3, AUTD3_TRUE};
    use datagram::AUTDDatagramTuple;

    use super::*;

    #[test]
    fn simple() {
        unsafe {
            let pos = [Vector3::new(0., 0., 0.)];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let builder = controller::builder::AUTDControllerBuilder(pos.as_ptr(), rot.as_ptr(), 1);
            let link_builder = link::nop::AUTDLinkNop();
            let cnt = controller::builder::AUTDControllerOpen(builder, link_builder, -1);
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;

            let g = gain::focus::AUTDGainFocus(Vector3::new(0., 0., 150.), 0xFF, 0x00);
            let m = modulation::r#static::AUTDModulationStatic(
                0xFF,
                driver::firmware::fpga::loop_behavior::AUTDLoopBehaviorInfinite(),
            );

            let d1 = gain::AUTDGainIntoDatagram(g);
            let d2 = modulation::AUTDModulationIntoDatagram(m);
            let d = AUTDDatagramTuple(d1, d2);
            assert_eq!(AUTD3_TRUE, controller::AUTDControllerSend(cnt, d).result);

            assert_eq!(AUTD3_TRUE, controller::AUTDControllerClose(cnt).result);

            controller::AUTDControllerDelete(cnt);
        }
    }
}
