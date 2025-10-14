use crate::impl_ptr;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GeometryPtr(pub *const std::ffi::c_void);

impl_ptr!(GeometryPtr, autd3::driver::geometry::Geometry);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DevicePtr(pub *const std::ffi::c_void);

impl_ptr!(DevicePtr, autd3::driver::geometry::Device);

#[repr(C)]
pub struct TransducerPtr(pub *const std::ffi::c_void);

impl_ptr!(TransducerPtr, autd3::driver::geometry::Transducer);
