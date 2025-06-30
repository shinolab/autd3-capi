use crate::impl_ptr;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EnvironmentPtr(pub *const libc::c_void);

impl_ptr!(EnvironmentPtr, autd3::core::environment::Environment);
