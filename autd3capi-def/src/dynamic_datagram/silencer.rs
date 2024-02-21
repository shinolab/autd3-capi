use std::time::Duration;

use autd3_driver::{
    datagram::{ConfigureSilencerFixedCompletionSteps, ConfigureSilencerFixedUpdateRate, Datagram},
    error::AUTDInternalError,
    operation::Operation,
};

use crate::DynamicDatagram;

impl DynamicDatagram for ConfigureSilencerFixedUpdateRate {
    #[allow(clippy::box_default)]
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::new(<Self as Datagram>::O1::new(
                self.update_rate_intensity(),
                self.update_rate_phase(),
            )),
            Box::new(<Self as Datagram>::O2::default()),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}

impl DynamicDatagram for ConfigureSilencerFixedCompletionSteps {
    #[allow(clippy::box_default)]
    fn operation(&mut self) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        Ok((
            Box::new(<Self as Datagram>::O1::new(
                self.completion_steps_intensity(),
                self.completion_steps_phase(),
                self.strict_mode(),
            )),
            Box::new(<Self as Datagram>::O2::default()),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }
}
