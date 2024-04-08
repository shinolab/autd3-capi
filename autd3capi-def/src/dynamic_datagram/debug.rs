use std::{collections::HashMap, time::Duration};

use super::DynamicDatagram;
use autd3::prelude::*;
use autd3_driver::{
    error::AUTDInternalError,
    operation::{Operation, TypeTag},
};

#[repr(C, align(2))]
struct DebugSettingInner {
    tag: TypeTag,
    __pad: u8,
    ty: [u8; 4],
    value: [u16; 4],
}

#[repr(C)]
#[derive(Clone)]
pub struct DebugSettings {
    pub ty: [u8; 4],
    pub value: [u16; 4],
}

pub struct DynamicConfigureDebugSettings {
    map: HashMap<usize, DebugSettings>,
}

impl DynamicConfigureDebugSettings {
    pub fn new(map: HashMap<usize, DebugSettings>) -> Self {
        Self { map }
    }
}

pub struct DynamicDebugSettingOp {
    remains: HashMap<usize, usize>,
    idx_map: HashMap<usize, DebugSettings>,
}

impl Operation for DynamicDebugSettingOp {
    fn pack(&mut self, device: &Device, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);
        let d = unsafe {
            (tx.as_mut_ptr() as *mut DebugSettingInner)
                .as_mut()
                .unwrap()
        };
        d.tag = TypeTag::Debug;
        let v = &self.idx_map[&device.idx()];
        for (i, &ty) in v.ty.iter().enumerate() {
            d.ty[i] = ty;
        }
        for (i, &value) in v.value.iter().enumerate() {
            d.value[i] = value;
        }
        Ok(std::mem::size_of::<DebugSettingInner>())
    }

    fn required_size(&self, _: &Device) -> usize {
        std::mem::size_of::<DebugSettingInner>()
    }

    fn init(&mut self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        self.remains = geometry.devices().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device) {
        self.remains.insert(device.idx(), 0);
    }
}

impl DynamicDatagram for DynamicConfigureDebugSettings {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::new(DynamicDebugSettingOp {
                remains: Default::default(),
                idx_map: self.map.clone(),
            }),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}
