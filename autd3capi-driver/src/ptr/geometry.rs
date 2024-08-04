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

impl std::ops::Deref for DevicePtr {
    type Target = autd3_driver::geometry::Device;
    fn deref(&self) -> &Self::Target {
        unsafe {
            (self.0 as *const autd3_driver::geometry::Device)
                .as_ref()
                .unwrap()
        }
    }
}

#[repr(C)]
pub struct TransducerPtr(pub *const libc::c_void);

impl std::ops::Deref for TransducerPtr {
    type Target = autd3_driver::geometry::Transducer;
    fn deref(&self) -> &Self::Target {
        unsafe {
            (self.0 as *const autd3_driver::geometry::Transducer)
                .as_ref()
                .unwrap()
        }
    }
}
