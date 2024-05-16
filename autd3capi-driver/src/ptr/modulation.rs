use autd3_driver::derive::Modulation;

use crate::{ConstPtr, M};

#[repr(C)]
pub struct ModulationPtr(pub ConstPtr);

impl<T: Modulation + 'static> From<T> for ModulationPtr {
    fn from(m: T) -> Self {
        let m: Box<Box<M>> = Box::new(Box::new(m));
        Self(Box::into_raw(m) as _)
    }
}

#[macro_export]
macro_rules! take_mod {
    ($ptr:expr, $type:ty) => {
        Box::from_raw($ptr.0 as *mut Box<M> as *mut Box<$type>)
    };
}
