#[derive(Clone, Copy)]
#[repr(C)]
pub struct Duration {
    pub nanos: u64,
}

impl From<std::time::Duration> for Duration {
    fn from(d: std::time::Duration) -> Self {
        Self {
            nanos: d.as_nanos() as u64,
        }
    }
}

impl From<Duration> for std::time::Duration {
    fn from(d: Duration) -> Self {
        Self::from_nanos(d.nanos)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OptionDuration {
    pub has_value: bool,
    pub value: Duration,
}

impl OptionDuration {
    pub const NONE: Self = Self {
        has_value: false,
        value: Duration { nanos: 0 },
    };
}

impl From<OptionDuration> for Option<std::time::Duration> {
    fn from(d: OptionDuration) -> Self {
        if d.has_value {
            Some(d.value.into())
        } else {
            None
        }
    }
}
