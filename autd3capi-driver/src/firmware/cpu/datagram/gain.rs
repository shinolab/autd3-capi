use std::time::Duration;

use autd3::{derive::DatagramS, gain::Null};
use autd3_driver::firmware::operation::Operation;

use crate::{DynamicDatagramS, Segment, G};

impl DynamicDatagramS for Box<G> {
    fn operation_with_segment(
        &mut self,
        segment: Segment,
        update_segment: bool,
    ) -> (Box<dyn Operation>, Box<dyn Operation>) {
        let mut tmp: Box<G> = Box::<Null>::default();
        std::mem::swap(&mut tmp, self);
        (
            Box::new(autd3_driver::firmware::operation::GainOp::new(
                segment.into(),
                update_segment,
                tmp,
            )),
            Box::<autd3_driver::firmware::operation::NullOp>::default(),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as DatagramS>::timeout(self)
    }
}
