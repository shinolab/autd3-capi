use std::sync::Mutex;

use crate::{autd3::derive::*, M};

use derive_more::{Debug, Deref};

#[derive(Modulation, Clone, Deref, Debug)]
pub struct BoxedCache {
    #[deref]
    pub m: Arc<M>,
    #[debug("{}", !self.cache.lock().unwrap().is_empty())]
    cache: Arc<Mutex<Arc<Vec<u8>>>>,
    #[no_change]
    config: SamplingConfig,
    loop_behavior: LoopBehavior,
}

impl BoxedCache {
    pub fn new(m: Box<M>) -> Self {
        Self {
            config: m.sampling_config(),
            loop_behavior: m.loop_behavior(),
            m: Arc::from(m),
            cache: Default::default(),
        }
    }
}

impl Modulation for BoxedCache {
    fn calc(&self) -> Result<Arc<Vec<u8>>, AUTDInternalError> {
        if self.cache.lock().unwrap().is_empty() {
            tracing::debug!("Initialize cache");
            *self.cache.lock().unwrap() = self.m.calc()?;
        }
        let buffer = self.cache.lock().unwrap().clone();
        Ok(buffer)
    }
}
