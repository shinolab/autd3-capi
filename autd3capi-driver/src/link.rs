use std::time::Duration;

use async_ffi::BorrowingFfiFuture;

use autd3_driver::{
    async_trait,
    error::AUTDInternalError,
    firmware::cpu::{RxMessage, TxDatagram},
    geometry::Geometry,
    link::{Link, LinkBuilder},
};

use crate::{impl_ptr, L};

#[repr(C)]
pub struct LinkBuilderPtr(pub *const libc::c_void);

#[repr(C)]
pub struct LinkPtr(pub *const libc::c_void);

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

pub struct SyncLink<T: Link> {
    pub runtime: tokio::runtime::Runtime,
    pub inner: T,
}

#[async_trait]
impl<T: Link> Link for SyncLink<T> {
    async fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.runtime.block_on(self.inner.close())
    }

    async fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        self.runtime.block_on(self.inner.send(tx))
    }

    async fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        self.runtime.block_on(self.inner.receive(rx))
    }

    #[must_use]
    fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    fn timeout(&self) -> Duration {
        self.inner.timeout()
    }

    #[inline(always)]
    fn trace(&mut self, tx: &TxDatagram, rx: &mut [RxMessage], timeout: Option<Duration>) {
        self.inner.trace(tx, rx, timeout)
    }
}

pub struct SyncLinkBuilder<L: Link, T: LinkBuilder<L = L>> {
    pub runtime: tokio::runtime::Runtime,
    pub inner: T,
}

#[async_trait]
impl<L: Link, T: LinkBuilder<L = L>> LinkBuilder for SyncLinkBuilder<L, T> {
    type L = SyncLink<L>;

    async fn open(self, geometry: &Geometry) -> Result<Self::L, AUTDInternalError> {
        let inner = self.runtime.block_on(self.inner.open(geometry))?;
        Ok(SyncLink {
            runtime: self.runtime,
            inner,
        })
    }
}
