mod controller;
mod datagram;
mod environment;
mod gain;
mod geometry;
mod link;
mod modulation;
mod stm;

pub use controller::*;
pub use datagram::*;
pub use environment::*;
pub use gain::*;
pub use geometry::*;
pub use link::*;
pub use modulation::*;
pub use stm::*;

pub trait CapiResult {
    const NULL: Self;
}

#[macro_export]
macro_rules! impl_ptr {
    ($name:ident) => {
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $name {
            pub const NULL: Self = Self(std::ptr::null());
        }
    };
    ($name:ident, $type:ty) => {
        impl_ptr!($name);

        impl From<$type> for $name {
            fn from(v: $type) -> Self {
                Self(Box::into_raw(Box::new(v)) as _)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $type;
            fn deref(&self) -> &Self::Target {
                unsafe { (self.0 as *const $type).as_ref().unwrap() }
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { (self.0 as *mut $type).as_mut().unwrap() }
            }
        }
    };
}
