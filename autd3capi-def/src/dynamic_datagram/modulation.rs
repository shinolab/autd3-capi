use std::time::Duration;

use autd3_driver::{
    datagram::{ChangeModulationSegment, Datagram},
    error::AUTDInternalError,
    operation::Operation,
};

use crate::{DynamicDatagram, DynamicDatagramS, Segment, M};

impl DynamicDatagramS for Box<M> {
    fn operation_with_segment(
        &mut self,
        segment: Segment,
        update_segment: bool,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let freq_div = self.sampling_config().frequency_division();
        let buf = self.calc()?;
        let loop_behavior = self.loop_behavior();
        Ok((
            Box::new(autd3_driver::operation::ModulationOp::new(
                buf,
                freq_div,
                loop_behavior,
                segment.into(),
                update_segment,
            )),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for ChangeModulationSegment {
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
