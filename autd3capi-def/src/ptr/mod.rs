mod datagram;
mod gain;
mod geometry;
mod modulation;
mod stm;

pub use datagram::*;
pub use gain::*;
pub use geometry::*;
pub use modulation::*;
pub use stm::*;

#[macro_export]
macro_rules! impl_ptr {
    ($name:ident, $type:ty) => {
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
