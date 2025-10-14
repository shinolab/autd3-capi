use crate::{ConstPtr, impl_ptr, impl_result};

use autd3::{core::modulation::Modulation, driver::datagram::BoxedModulation};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ModulationPtr(pub *const std::ffi::c_void);

impl_ptr!(ModulationPtr);

impl<T: Modulation + 'static> From<T> for ModulationPtr {
    fn from(m: T) -> Self {
        Self(Box::into_raw(Box::new(BoxedModulation::new(m))) as _)
    }
}

#[repr(C)]
pub struct ResultModulation {
    pub result: ModulationPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_result!(ResultModulation, ModulationPtr);
