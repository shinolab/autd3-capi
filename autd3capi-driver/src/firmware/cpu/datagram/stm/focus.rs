use std::time::Duration;

use autd3_driver::{
    datagram::{Datagram, FocusSTM},
    firmware::fpga::TransitionMode,
};

use crate::{DynamicDatagramST, Segment};

impl DynamicDatagramST for FocusSTM {
    fn operation_with_segment(
        &mut self,
        segment: Segment,
        transition_mode: Option<TransitionMode>,
    ) -> (
        Box<dyn autd3_driver::firmware::operation::Operation>,
        Box<dyn autd3_driver::firmware::operation::Operation>,
    ) {
        let config = self.stm_sampling_config();
        let loop_behavior = self.loop_behavior();
        (
            Box::new(<Self as Datagram>::O1::new(
                self.clear(),
                config,
                loop_behavior,
                segment.into(),
                transition_mode,
            )),
            Box::<autd3_driver::firmware::operation::NullOp>::default(),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
