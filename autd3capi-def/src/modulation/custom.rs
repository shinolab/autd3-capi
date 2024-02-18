use autd3_driver::derive::*;

#[derive(Modulation)]
pub struct CustomModulation {
    pub buf: Vec<EmitIntensity>,
    pub config: SamplingConfiguration,
    pub loop_behavior: LoopBehavior,
}

impl autd3_driver::datagram::Modulation for CustomModulation {
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}
