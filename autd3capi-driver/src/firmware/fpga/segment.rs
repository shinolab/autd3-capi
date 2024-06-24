#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Segment {
    S0 = 0,
    S1 = 1,
}

impl From<Segment> for autd3::prelude::Segment {
    fn from(segment: Segment) -> Self {
        match segment {
            Segment::S0 => autd3::prelude::Segment::S0,
            Segment::S1 => autd3::prelude::Segment::S1,
        }
    }
}

impl From<autd3::prelude::Segment> for Segment {
    fn from(segment: autd3::prelude::Segment) -> Self {
        match segment {
            autd3::prelude::Segment::S0 => Segment::S0,
            autd3::prelude::Segment::S1 => Segment::S1,
            _ => unimplemented!(),
        }
    }
}
