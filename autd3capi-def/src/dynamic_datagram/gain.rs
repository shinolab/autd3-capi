use std::time::Duration;

use autd3::gain::Null;
use autd3_driver::{
    datagram::{ChangeGainSegment, Datagram},
    operation::Operation,
};

use crate::{DynamicDatagram, DynamicDatagramS, Segment, G};

impl DynamicDatagramS for Box<G> {
    fn operation_with_segment(
        &mut self,
        segment: Segment,
        update_segment: bool,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), autd3::prelude::AUTDInternalError> {
        let mut tmp: Box<G> = Box::<Null>::default();
        std::mem::swap(&mut tmp, self);
        Ok((
            Box::new(autd3_driver::operation::GainOp::new(
                segment.into(),
                update_segment,
                tmp,
            )),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for ChangeGainSegment {
    fn operation(
        &mut self,
    ) -> Result<
        (
            Box<dyn autd3_driver::operation::Operation>,
            Box<dyn autd3_driver::operation::Operation>,
        ),
        autd3::prelude::AUTDInternalError,
    > {
        Ok((
            Box::new(<Self as Datagram>::O1::new(self.segment())),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
