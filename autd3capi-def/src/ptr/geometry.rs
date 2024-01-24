use crate::{impl_ptr, ConstPtr};

impl_ptr!(GeometryPtr, autd3_driver::geometry::Geometry);
impl_ptr!(DevicePtr, autd3_driver::geometry::Device);
impl_ptr!(TransducerPtr, autd3_driver::geometry::Transducer);
