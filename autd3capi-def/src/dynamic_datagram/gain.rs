use std::time::Duration;

use autd3::gain::Null;
use autd3_driver::operation::Operation;

use crate::{DynamicDatagramS, G};

impl DynamicDatagramS for Box<G> {
    fn operation_with_segment(
        &mut self,
        segment: autd3::prelude::Segment,
        update_segment: bool,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), autd3::prelude::AUTDInternalError> {
        let mut tmp: Box<G> = Box::<Null>::default();
        std::mem::swap(&mut tmp, self);
        Ok((
            Box::new(autd3_driver::operation::GainOp::new(
                segment,
                update_segment,
                tmp,
            )),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}
