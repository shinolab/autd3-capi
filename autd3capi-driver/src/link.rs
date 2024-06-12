use async_ffi::BorrowingFfiFuture;

use autd3_driver::{
    error::AUTDInternalError,
    geometry::Geometry,
    link::{Link, LinkBuilder},
};

use crate::{impl_ptr, ConstPtr, L};

#[repr(C)]
pub struct LinkBuilderPtr(pub ConstPtr);

#[repr(C)]
pub struct LinkPtr(pub ConstPtr);

unsafe impl Send for LinkPtr {}
unsafe impl Sync for LinkPtr {}

impl_ptr!(LinkPtr, Box<L>);

impl LinkPtr {
    pub fn cast<T: Link>(&self) -> &T {
        unsafe { (self.0 as *const Box<L> as *const Box<T>).as_ref().unwrap() }
    }

    pub fn cast_mut<T: Link>(&mut self) -> &mut T {
        unsafe { (self.0 as *mut Box<L> as *mut Box<T>).as_mut().unwrap() }
    }
}

pub struct DynamicLinkBuilder {
    #[allow(clippy::type_complexity)]
    gen: Box<dyn FnOnce(&Geometry) -> BorrowingFfiFuture<Result<Box<L>, AUTDInternalError>>>,
}

unsafe impl Send for DynamicLinkBuilder {}
unsafe impl Sync for DynamicLinkBuilder {}

impl DynamicLinkBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<B: LinkBuilder + 'static>(builder: B) -> LinkBuilderPtr
    where
        B::L: Link,
    {
        LinkBuilderPtr(Box::into_raw(Box::new(Self {
            gen: Box::new(move |geometry| {
                BorrowingFfiFuture::new(async move {
                    let r: Result<Box<L>, AUTDInternalError> = match builder.open(geometry).await {
                        Ok(v) => Ok(Box::new(v)),
                        Err(e) => Err(e),
                    };
                    r
                })
            }),
        })) as _)
    }
}

#[autd3_driver::async_trait]
impl LinkBuilder for DynamicLinkBuilder {
    type L = Box<L>;

    async fn open(self, geometry: &Geometry) -> Result<Self::L, AUTDInternalError> {
        (self.gen)(geometry).await
    }
}
