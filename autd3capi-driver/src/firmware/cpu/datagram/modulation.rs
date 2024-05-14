use std::time::Duration;

use autd3::modulation::Static;
use autd3_driver::{datagram::Datagram, firmware::operation::Operation};

use crate::{DynamicDatagramST, Segment, TransitionMode, M};

impl DynamicDatagramST for Box<M> {
    fn operation_with_segment(
        &mut self,
        segment: Segment,
        transition_mode: Option<TransitionMode>,
    ) -> (Box<dyn Operation>, Box<dyn Operation>) {
        let mut tmp: Box<M> = Box::<Static>::default();
        std::mem::swap(&mut tmp, self);
        (
            Box::new(autd3_driver::firmware::operation::ModulationOp::new(
                tmp,
                segment.into(),
                transition_mode.map(|m| m.into()),
            )),
            Box::<autd3_driver::firmware::operation::NullOp>::default(),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
