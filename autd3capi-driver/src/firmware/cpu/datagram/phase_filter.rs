use std::time::Duration;

use crate::{ConstPtr, GeometryPtr};

use super::DynamicDatagram;
use autd3::{derive::DEFAULT_TIMEOUT, prelude::*};
use autd3_driver::firmware::operation::Operation;

pub struct DynamicPhaseFilter {
    pub f: ConstPtr,
    pub context: ConstPtr,
    pub geometry: GeometryPtr,
}

impl DynamicDatagram for DynamicPhaseFilter {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        unsafe {
            let f = std::mem::transmute::<
                _,
                unsafe extern "C" fn(ConstPtr, GeometryPtr, u32, u8) -> Phase,
            >(self.f);
            let context = self.context;
            let geometry = self.geometry;
            (
                Box::new(autd3_driver::firmware::operation::PhaseFilterOp::new(
                    move |dev| {
                        let dev_idx = dev.idx() as u32;
                        move |tr| f(context, geometry, dev_idx, tr.idx() as u8)
                    },
                )),
                Box::<autd3_driver::firmware::operation::NullOp>::default(),
            )
        }
    }

    fn timeout(&self) -> Option<Duration> {
        Some(DEFAULT_TIMEOUT)
    }
}
