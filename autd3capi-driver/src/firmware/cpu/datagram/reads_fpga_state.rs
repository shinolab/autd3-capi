use std::time::Duration;

use crate::{ConstPtr, GeometryPtr};

use super::DynamicDatagram;
use autd3::derive::DEFAULT_TIMEOUT;
use autd3_driver::firmware::operation::Operation;

pub struct DynamicReadsFPGAState {
    pub f: ConstPtr,
    pub context: ConstPtr,
    pub geometry: GeometryPtr,
}

impl DynamicDatagram for DynamicReadsFPGAState {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        unsafe {
            let f = std::mem::transmute::<
                _,
                unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u32) -> bool,
            >(self.f);
            let context = self.context;
            let geometry = self.geometry;
            (
                Box::new(autd3_driver::firmware::operation::ReadsFPGAStateOp::new(
                    move |dev| f(context, geometry, dev.idx() as _),
                )),
                Box::<autd3_driver::firmware::operation::NullOp>::default(),
            )
        }
    }

    fn timeout(&self) -> Option<Duration> {
        Some(DEFAULT_TIMEOUT)
    }
}
