#[repr(C)]
pub struct Range {
    pub x_start: f32,
    pub x_end: f32,
    pub y_start: f32,
    pub y_end: f32,
    pub z_start: f32,
    pub z_end: f32,
    pub resolution: f32,
}

impl From<Range> for autd3_emulator::Range {
    fn from(value: Range) -> Self {
        autd3_emulator::Range {
            x: value.x_start..=value.x_end,
            y: value.y_start..=value.y_end,
            z: value.z_start..=value.z_end,
            resolution: value.resolution,
        }
    }
}
