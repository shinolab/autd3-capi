use std::time::Duration;

use autd3_driver::firmware::operation::Operation;

use crate::{DynamicDatagram, Segment, TransitionMode};

pub struct DynamicDatagramWithSegmentTransition<D: DynamicDatagramST> {
    datagram: D,
    segment: Segment,
    transition_mode: Option<TransitionMode>,
}

pub trait DynamicDatagramST {
    #[allow(clippy::type_complexity)]
    fn operation_with_segment(
        &mut self,
        segment: Segment,
        transition_mode: Option<TransitionMode>,
    ) -> (Box<dyn Operation>, Box<dyn Operation>);

    fn timeout(&self) -> Option<Duration>;
}

impl<D: DynamicDatagramST> DynamicDatagram for DynamicDatagramWithSegmentTransition<D> {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        self.datagram
            .operation_with_segment(self.segment, self.transition_mode)
    }

    fn timeout(&self) -> Option<Duration> {
        self.datagram.timeout()
    }
}

impl<D: DynamicDatagramST> DynamicDatagram for D {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        <Self as DynamicDatagramST>::operation_with_segment(
            self,
            Segment::S0,
            Some(autd3_driver::firmware::fpga::TransitionMode::Immediate.into()),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as DynamicDatagramST>::timeout(self)
    }
}

pub trait IntoDynamicDatagramWithSegmentTransition<D: DynamicDatagramST> {
    /// Set segment
    fn with_segment(
        self,
        segment: Segment,
        transition_mode: Option<TransitionMode>,
    ) -> DynamicDatagramWithSegmentTransition<D>;
}

impl<D: DynamicDatagramST> IntoDynamicDatagramWithSegmentTransition<D> for D {
    fn with_segment(
        self,
        segment: Segment,
        transition_mode: Option<TransitionMode>,
    ) -> DynamicDatagramWithSegmentTransition<D> {
        DynamicDatagramWithSegmentTransition {
            datagram: self,
            segment,
            transition_mode,
        }
    }
}
