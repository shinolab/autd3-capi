use std::sync::Mutex;

use crate::autd3::derive::*;

use autd3_driver::datagram::BoxedModulation;
use derive_more::{Debug, Deref};

#[derive(Modulation, Clone, Deref, Debug)]
pub struct BoxedCache {
    #[deref]
    pub m: Arc<Mutex<Option<BoxedModulation>>>,
    #[debug("{}", !self.cache.lock().unwrap().is_empty())]
    pub cache: Arc<Mutex<Vec<u8>>>,
    #[no_change]
    pub config: SamplingConfig,
    pub loop_behavior: LoopBehavior,
}

impl BoxedCache {
    pub fn new(m: BoxedModulation) -> Self {
        Self {
            config: m.sampling_config(),
            loop_behavior: m.loop_behavior(),
            m: Arc::new(Mutex::new(Some(m))),
            cache: Default::default(),
        }
    }
}

impl Modulation for BoxedCache {
    fn calc(self) -> Result<Vec<u8>, AUTDInternalError> {
        if let Some(m) = self.m.lock().unwrap().take() {
            tracing::debug!("Initializing cache");
            *self.cache.lock().unwrap() = m.calc()?;
        }
        let buffer = self.cache.lock().unwrap().clone();
        Ok(buffer)
    }
}
