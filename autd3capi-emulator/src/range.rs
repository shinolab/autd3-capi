#[repr(C)]
pub struct RangeXYZ {
    pub x_start: f32,
    pub x_end: f32,
    pub y_start: f32,
    pub y_end: f32,
    pub z_start: f32,
    pub z_end: f32,
    pub resolution: f32,
}

impl From<RangeXYZ> for autd3_emulator::RangeXYZ {
    fn from(value: RangeXYZ) -> Self {
        autd3_emulator::RangeXYZ {
            x: value.x_start..=value.x_end,
            y: value.y_start..=value.y_end,
            z: value.z_start..=value.z_end,
            resolution: value.resolution,
        }
    }
}
