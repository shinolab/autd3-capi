use std::time::Duration;

use crate::{ConstPtr, DebugSetting, GPIOOut, GeometryPtr};

use super::DynamicDatagram;
use autd3::derive::DEFAULT_TIMEOUT;
use autd3_driver::firmware::operation::Operation;

pub struct DynamicDebugSettings {
    pub f: ConstPtr,
    pub context: ConstPtr,
    pub geometry: GeometryPtr,
}

impl DynamicDatagram for DynamicDebugSettings {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        unsafe {
            let f = std::mem::transmute::<
                _,
                unsafe extern "C" fn(
                    ConstPtr,
                    geometry: GeometryPtr,
                    u32,
                    GPIOOut,
                    *mut DebugSetting,
                ),
            >(self.f);
            let context = self.context;
            let geometry = self.geometry;
            (
                Box::new(autd3_driver::firmware::operation::DebugSettingOp::new(
                    move |dev, gpio_out| {
                        let mut debug_setting = DebugSetting {
                            ty: crate::DebugType::None,
                            value: 0,
                        };
                        f(
                            context,
                            geometry,
                            dev.idx() as u32,
                            gpio_out.into(),
                            &mut debug_setting as *mut _,
                        );
                        debug_setting.convert(dev)
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
