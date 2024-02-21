use std::{collections::HashMap, time::Duration};

use super::DynamicDatagram;
use autd3::prelude::*;
use autd3_driver::{
    error::AUTDInternalError,
    operation::{ConfigurePhaseFilterOp, Operation},
};

type Op = ConfigurePhaseFilterOp<Box<dyn Fn(&Device, &Transducer) -> Phase>>;

pub struct DynamicConfigurePhaseFilter {
    map: HashMap<usize, HashMap<usize, Phase>>,
}

impl DynamicConfigurePhaseFilter {
    pub fn additive(map: HashMap<usize, HashMap<usize, Phase>>) -> Self {
        Self { map }
    }
}

pub struct DynamicConfigurePhaseFilterOp {
    op: Op,
}

impl Operation for DynamicConfigurePhaseFilterOp {
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

impl DynamicDatagram for DynamicConfigurePhaseFilter {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let map = self.map.clone();
        Ok((
            Box::new(DynamicConfigurePhaseFilterOp {
                op: ConfigurePhaseFilterOp::new(Box::new(move |dev, tr| {
                    map[&dev.idx()][&tr.idx()]
                })),
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
        let mut datagram = DynamicConfigurePhaseFilter::additive(Default::default());
        assert_eq!(
            datagram.timeout(),
            ConfigurePhaseFilter::additive(|_, _| Phase::new(0)).timeout()
        );
        let _ = datagram.operation();
    }
}
