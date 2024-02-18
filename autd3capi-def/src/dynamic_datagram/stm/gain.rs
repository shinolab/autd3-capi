use std::time::Duration;

use autd3_driver::datagram::{Datagram, GainSTM};

use crate::{DynamicDatagramS, Segment, G};

impl DynamicDatagramS for GainSTM<Box<G>> {
    fn operation_with_segment(
        &mut self,
        segment: Segment,
        update_segment: bool,
    ) -> Result<
        (
            Box<dyn autd3_driver::operation::Operation>,
            Box<dyn autd3_driver::operation::Operation>,
        ),
        autd3::prelude::AUTDInternalError,
    > {
        let freq_div = self.sampling_config()?.frequency_division();
        let loop_behavior = self.loop_behavior();
        Ok((
            Box::new(<Self as Datagram>::O1::new(
                self.clear(),
                self.mode(),
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
