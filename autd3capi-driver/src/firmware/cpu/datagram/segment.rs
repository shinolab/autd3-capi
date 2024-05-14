use std::time::Duration;

use autd3::derive::Datagram;
use autd3_driver::{
    datagram::{segment::SwapSegmentDatagram, SwapSegment},
    firmware::operation::SwapSegmentOperation,
};

use crate::DynamicDatagram;

impl<T: SwapSegmentDatagram> DynamicDatagram for SwapSegment<T>
where
    <T as SwapSegmentDatagram>::O: 'static,
{
    fn operation(
        &mut self,
    ) -> (
        Box<dyn autd3_driver::firmware::operation::Operation>,
        Box<dyn autd3_driver::firmware::operation::Operation>,
    ) {
        (
            Box::new(<Self as Datagram>::O1::new(
                self.segment(),
                self.transition_mode(),
            )),
            Box::new(<Self as Datagram>::O2::default()),
        )
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
