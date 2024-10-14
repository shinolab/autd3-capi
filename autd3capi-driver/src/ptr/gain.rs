use autd3_driver::derive::Gain;

use crate::G;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GainPtr(pub *const libc::c_void);

impl<T: Gain + Send + Sync + 'static> From<T> for GainPtr {
    fn from(g: T) -> Self {
        let g: Box<Box<G>> = Box::new(Box::new(g));
        Self(Box::into_raw(g) as _)
    }
}

#[macro_export]
macro_rules! take_gain {
    ($ptr:expr, $type:ty) => {
        Box::from_raw($ptr.0 as *mut Box<G> as *mut Box<$type>)
    };
}
