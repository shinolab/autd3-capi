use crate::{impl_ptr, ConstPtr};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GeometryPtr(pub ConstPtr);

impl_ptr!(GeometryPtr, autd3_driver::geometry::Geometry);

unsafe impl Send for GeometryPtr {}
unsafe impl Sync for GeometryPtr {}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DevicePtr(pub ConstPtr);

impl_ptr!(DevicePtr, autd3_driver::geometry::Device);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TransducerPtr(pub ConstPtr);

impl_ptr!(TransducerPtr, autd3_driver::geometry::Transducer);
