use autd3_driver::{
    async_trait,
    error::AUTDInternalError,
    firmware::cpu::{RxMessage, TxMessage},
    geometry::Geometry,
    link::{Link, LinkBuilder},
};

use crate::{impl_ffi_result, impl_ptr, ConstPtr};

use super::DynamicLinkBuilder;

#[repr(C)]
pub struct SyncLinkBuilderPtr(pub *const libc::c_void);

impl_ptr!(SyncLinkBuilderPtr);

pub struct SyncLink<T: Link> {
    pub runtime: tokio::runtime::Runtime,
    pub inner: T,
}

#[async_trait]
impl<T: Link> Link for SyncLink<T> {
    async fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.runtime.block_on(self.inner.close())
    }

    async fn send(&mut self, tx: &[TxMessage]) -> Result<bool, AUTDInternalError> {
        self.runtime.block_on(self.inner.send(tx))
    }

    async fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        self.runtime.block_on(self.inner.receive(rx))
    }

    #[must_use]
    fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    async fn update(&mut self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        self.runtime.block_on(self.inner.update(geometry))
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

impl<B: LinkBuilder + 'static> From<B> for SyncLinkBuilderPtr
where
    B::L: Link,
{
    fn from(value: B) -> Self {
        let builder = if cfg!(feature = "static") {
            Box::into_raw(Box::new(DynamicLinkBuilder::new(value))) as _
        } else {
            Box::into_raw(Box::new(DynamicLinkBuilder::new(SyncLinkBuilder {
                runtime: tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap(),
                inner: value,
            }))) as _
        };
        SyncLinkBuilderPtr(builder)
    }
}

#[repr(C)]
pub struct ResultSyncLinkBuilder {
    pub result: SyncLinkBuilderPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_ffi_result!(ResultSyncLinkBuilder, SyncLinkBuilderPtr);
