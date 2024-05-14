use std::time::Duration;

use autd3_driver::firmware::operation::Operation;

use crate::{DynamicDatagram, Segment};

pub struct DynamicDatagramWithSegment<D: DynamicDatagramS> {
    datagram: D,
    segment: Segment,
    transition: bool,
}

pub trait DynamicDatagramS {
    #[allow(clippy::type_complexity)]
    fn operation_with_segment(
        &mut self,
        segment: Segment,
        transition: bool,
    ) -> (Box<dyn Operation>, Box<dyn Operation>);

    fn timeout(&self) -> Option<Duration>;
}

impl<D: DynamicDatagramS> DynamicDatagram for DynamicDatagramWithSegment<D> {
    fn operation(&mut self) -> (Box<dyn Operation>, Box<dyn Operation>) {
        self.datagram
            .operation_with_segment(self.segment, self.transition)
    }

    fn timeout(&self) -> Option<Duration> {
        self.datagram.timeout()
    }
}

pub trait IntoDynamicDatagramWithSegment<D: DynamicDatagramS> {
    /// Set segment
    fn with_segment(self, segment: Segment, transition: bool) -> DynamicDatagramWithSegment<D>;
}

impl<D: DynamicDatagramS> IntoDynamicDatagramWithSegment<D> for D {
    fn with_segment(self, segment: Segment, transition: bool) -> DynamicDatagramWithSegment<D> {
        DynamicDatagramWithSegment {
            datagram: self,
            segment,
            transition,
        }
    }
}
