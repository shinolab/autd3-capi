use autd3_driver::datagram::{BoxedGain, IntoBoxedGain};

use crate::{impl_result, impl_ptr, ConstPtr};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GainPtr(pub *const libc::c_void);

impl_ptr!(GainPtr);

impl<T: IntoBoxedGain + 'static> From<T> for GainPtr {
    fn from(g: T) -> Self {
        let g: Box<BoxedGain> = Box::new(g.into_boxed());
        Self(Box::into_raw(g) as _)
    }
}

#[repr(C)]
pub struct ResultGain {
    pub result: GainPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_result!(ResultGain, GainPtr);
