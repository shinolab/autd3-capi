use autd3_driver::datagram::IntoBoxedModulation;

use crate::{impl_ffi_result, impl_ptr, ConstPtr};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ModulationPtr(pub *const libc::c_void);

impl_ptr!(ModulationPtr);

impl<T: IntoBoxedModulation> From<T> for ModulationPtr {
    fn from(m: T) -> Self {
        Self(Box::into_raw(Box::new(m.into_boxed())) as _)
    }
}

#[repr(C)]
pub struct ResultModulation {
    pub result: ModulationPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_ffi_result!(ResultModulation, ModulationPtr);
