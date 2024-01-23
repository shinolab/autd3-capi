use std::{collections::HashMap, time::Duration};

use super::DynamicDatagram;
use autd3::prelude::*;
use autd3_driver::{
    error::AUTDInternalError,
    operation::{ConfigureReadsFPGAStateOp, Operation},
};

type Op = ConfigureReadsFPGAStateOp<Box<dyn Fn(&Device) -> bool>>;

pub struct DynamicConfigureReadsFPGAState {
    map: HashMap<usize, bool>,
}

impl DynamicConfigureReadsFPGAState {
    pub fn new(map: HashMap<usize, bool>) -> Self {
        Self { map }
    }
}

pub struct DynamicConfigureReadsFPGAStateOp {
    op: Op,
}

impl Operation for DynamicConfigureReadsFPGAStateOp {
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

impl DynamicDatagram for DynamicConfigureReadsFPGAState {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let map = self.map.clone();
        Ok((
            Box::new(DynamicConfigureReadsFPGAStateOp {
                op: ConfigureReadsFPGAStateOp::new(Box::new(move |dev: &Device| map[&dev.idx()])),
            }),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use autd3_driver::datagram::Datagram;

    #[test]
    fn test_dynamic_configure_reads_fpga_state() {
        let mut datagram = DynamicConfigureReadsFPGAState::new(Default::default());
        assert_eq!(
            datagram.timeout(),
            ConfigureReadsFPGAState::new(|_| false).timeout()
        );
        let _ = datagram.operation();
    }
}
