use std::time::Duration;

use autd3_driver::{error::AUTDInternalError, operation::Operation};

use crate::{DynamicDatagramS, M};

impl DynamicDatagramS for Box<M> {
    fn operation_with_segment(
        &mut self,
        segment: autd3::prelude::Segment,
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
                segment,
                update_segment,
            )),
            Box::<autd3_driver::operation::NullOp>::default(),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        Some(std::time::Duration::from_millis(200))
    }
}
