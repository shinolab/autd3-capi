use std::time::Duration;

use autd3_driver::{derive::Segment, error::AUTDInternalError, operation::Operation};

use crate::DynamicDatagram;

pub struct DynamicDatagramWithSegment<D: DynamicDatagramS> {
    datagram: D,
    segment: Segment,
    update_segment: bool,
}

pub trait DynamicDatagramS {
    fn operation_with_segment(
        &mut self,
        segment: Segment,
        update_segment: bool,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError>;

    fn timeout(&self) -> Option<Duration>;
}

// impl<D: DynamicDatagramS> DynamicDatagramWithSegment<D> {
//     pub const fn segment(&self) -> Segment {
//         self.segment
//     }

//     pub const fn update_segment(&self) -> bool {
//         self.update_segment
//     }
// }

// impl<D: DynamicDatagramS> std::ops::Deref for DynamicDatagramWithSegment<D> {
//     type Target = D;

//     fn deref(&self) -> &Self::Target {
//         &self.datagram
//     }
// }

impl<D: DynamicDatagramS> DynamicDatagram for DynamicDatagramWithSegment<D> {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        self.datagram
            .operation_with_segment(self.segment, self.update_segment)
    }

    fn timeout(&self) -> Option<Duration> {
        self.datagram.timeout()
    }
}

impl<D: DynamicDatagramS> DynamicDatagram for D {
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        <Self as DynamicDatagramS>::operation_with_segment(self, Segment::S0, true)
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as DynamicDatagramS>::timeout(self)
    }
}

pub trait IntoDynamicDatagramWithSegment<D: DynamicDatagramS> {
    /// Set segment
    fn with_segment(self, segment: Segment, update_segment: bool) -> DynamicDatagramWithSegment<D>;
}

impl<D: DynamicDatagramS> IntoDynamicDatagramWithSegment<D> for D {
    fn with_segment(self, segment: Segment, update_segment: bool) -> DynamicDatagramWithSegment<D> {
        DynamicDatagramWithSegment {
            datagram: self,
            segment,
            update_segment,
        }
    }
}
