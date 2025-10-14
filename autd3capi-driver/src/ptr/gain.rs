use autd3::core::gain::Gain;
use autd3::driver::datagram::BoxedGain;

use crate::{ConstPtr, impl_ptr, impl_result};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GainPtr(pub *const std::ffi::c_void);

impl_ptr!(GainPtr);

impl<T: Gain<'static> + 'static> From<T> for GainPtr {
    fn from(g: T) -> Self {
        let g: Box<BoxedGain> = Box::new(BoxedGain::new(g));
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
