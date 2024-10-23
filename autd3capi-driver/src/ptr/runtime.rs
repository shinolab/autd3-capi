use tokio::runtime::{Handle, Runtime};

use crate::impl_ptr;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RuntimePtr(pub *const libc::c_void);

impl_ptr!(RuntimePtr, Runtime);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HandlePtr(pub *const libc::c_void);

impl_ptr!(HandlePtr, Handle);
