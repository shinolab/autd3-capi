use std::time::Duration;

use crate::{ConstPtr, GeometryPtr};

use super::DynamicDatagram;
use autd3::derive::DEFAULT_TIMEOUT;
use autd3_driver::firmware::operation::Operation;

pub struct DynamicForceFan {
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
}

impl DynamicForceFan {
    pub const fn new(f: ConstPtr, context: ConstPtr, geometry: GeometryPtr) -> Self {
        Self {
            f,
            context,
            geometry,
        }
    }
}

impl DynamicDatagram for DynamicForceFan {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        unsafe {
            let f = std::mem::transmute::<
                _,
                unsafe extern "C" fn(ConstPtr, GeometryPtr, u32) -> bool,
            >(self.f);
            let context = self.context;
            let geometry = self.geometry;
            (
                Box::new(autd3_driver::firmware::operation::ForceFanOp::new(
                    move |dev| f(context, geometry, dev.idx() as u32),
                )),
                Box::new(autd3_driver::firmware::operation::NullOp::default()),
            )
        }
    }

    fn timeout(&self) -> Option<Duration> {
        Some(DEFAULT_TIMEOUT)
    }
}
