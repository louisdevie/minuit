pub struct Area {
    absx: i32,
    absy: i32,
    relx: f32,
    rely: f32,
    absw: i32,
    absh: i32,
    relw: f32,
    relh: f32,
}

impl Area {
    pub fn default() -> Self {
        Self {
            absx: 0,
            absy: 0,
            relx: 0.0,
            rely: 0.0,
            absw: 0,
            absh: 0,
            relw: 1.0,
            relh: 1.0,
        }
    }
}
