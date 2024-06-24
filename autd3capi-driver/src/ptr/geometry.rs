use crate::impl_ptr;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GeometryPtr(pub *const libc::c_void);

impl_ptr!(GeometryPtr, autd3_driver::geometry::Geometry);

unsafe impl Send for GeometryPtr {}
unsafe impl Sync for GeometryPtr {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DevicePtr(pub *const libc::c_void);

impl_ptr!(DevicePtr, autd3_driver::geometry::Device);

#[repr(C)]
pub struct TransducerPtr(pub *const libc::c_void);

impl_ptr!(TransducerPtr, autd3_driver::geometry::Transducer);
