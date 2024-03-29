use std::{collections::HashMap, time::Duration};

use super::DynamicDatagram;
use autd3::prelude::*;
use autd3_driver::{
    datagram::Datagram,
    error::AUTDInternalError,
    operation::{ConfigureForceFanOp, Operation},
};

type Op = ConfigureForceFanOp<Box<dyn Fn(&Device) -> bool>>;

pub struct DynamicConfigureForceFan {
    map: HashMap<usize, bool>,
}

impl DynamicConfigureForceFan {
    pub fn new(map: HashMap<usize, bool>) -> Self {
        Self { map }
    }
}

pub struct DynamicConfigureForceFanOp {
    op: Op,
}

impl Operation for DynamicConfigureForceFanOp {
    fn pack(&mut self, device: &Device, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        self.op.pack(device, tx)
    }

    fn required_size(&self, device: &Device) -> usize {
        self.op.required_size(device)
    }

    fn init(&mut self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        self.op.init(geometry)
    }

    fn remains(&self, device: &Device) -> usize {
        self.op.remains(device)
    }

    fn commit(&mut self, device: &Device) {
        self.op.commit(device)
    }
}

impl DynamicDatagram for DynamicConfigureForceFan {
    #[allow(clippy::box_default)]
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let map = self.map.clone();
        Ok((
            Box::new(DynamicConfigureForceFanOp {
                op: ConfigureForceFanOp::new(Box::new(move |dev: &Device| map[&dev.idx()])),
            }),
            Box::new(<ConfigureForceFan<Box<dyn Fn(&Device) -> bool>> as Datagram>::O2::default()),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}
