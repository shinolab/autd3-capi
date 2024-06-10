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
    use autd3capi_driver::{
        driver::{defined::ControlPoints, geometry::Quaternion},
        Segment, Vector3, AUTD3_TRUE,
    };
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

    #[test]
    fn foci() {
        unsafe {
            let pos = [Vector3::new(0., 0., 0.)];
            let rot = [Quaternion::new(1., 0., 0., 0.)];
            let builder = controller::builder::AUTDControllerBuilder(pos.as_ptr(), rot.as_ptr(), 1);
            let audit = link::audit::AUTDLinkAudit();
            let link_builder = link::audit::AUTDLinkAuditIntoBuilder(audit);
            let cnt = controller::builder::AUTDControllerOpen(builder, link_builder, -1);
            assert!(!cnt.result.0.is_null());
            let cnt = cnt.result;

            let geo = geometry::AUTDGeometry(cnt);
            let dev = geometry::device::AUTDDevice(geo, 0);

            let center = geometry::device::AUTDDeviceCenter(dev) + Vector3::new(0., 0., 150.);

            let points = [
                ControlPoints::from([center, center]).with_intensity(0x80),
                ControlPoints::from([center, center]).with_intensity(0x80),
            ];

            let d = datagram::stm::foci::AUTDSTMFociFromFreq(
                1.0,
                points.as_ptr() as _,
                points.len() as _,
                2,
            );
            let d = datagram::stm::foci::AUTDSTMFociIntoDatagram(d.result, 2);
            let res = controller::AUTDControllerSend(cnt, d);
            assert_eq!(AUTD3_TRUE, res.result);

            let link = link::AUTDLinkGet(cnt);
            let n = geometry::device::AUTDDeviceNumTransducers(dev);
            let mut intensities = vec![0; n as usize];
            let mut phases = vec![0; n as usize];
            link::audit::AUTDLinkAuditFpgaDrives(
                link,
                Segment::S0,
                0,
                0,
                intensities.as_mut_ptr(),
                phases.as_mut_ptr(),
            );

            for i in 0..n {
                assert_eq!(0x80, intensities[i as usize]);
            }

            assert_eq!(AUTD3_TRUE, controller::AUTDControllerClose(cnt).result);

            controller::AUTDControllerDelete(cnt);
        }
    }
}
