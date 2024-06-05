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
    use autd3capi_driver::AUTD3_TRUE;

    use super::*;

    #[test]
    fn simple() {
        unsafe {
            let params = [0., 0., 0., 0., 0., 0., 0.];
            let builder = controller::builder::AUTDControllerBuilder(params.as_ptr(), 1);
            let link_builder = link::nop::AUTDLinkNop();
            let cnt = controller::builder::AUTDControllerOpen(builder, link_builder, -1);
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;

            let g = gain::focus::AUTDGainFocus(0., 0., 150., 0xFF, 0x00);
            let m = modulation::r#static::AUTDModulationStatic(
                0xFF,
                driver::firmware::fpga::loop_behavior::AUTDLoopBehaviorInfinite(),
            );

            let d1 = gain::AUTDGainIntoDatagram(g);
            let d2 = modulation::AUTDModulationIntoDatagram(m);
            assert_eq!(
                AUTD3_TRUE,
                controller::AUTDControllerSend(cnt, d1, d2).result
            );

            assert_eq!(AUTD3_TRUE, controller::AUTDControllerClose(cnt).result);

            controller::AUTDControllerDelete(cnt);
        }
    }
}
