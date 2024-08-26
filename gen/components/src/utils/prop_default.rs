#[derive(Clone)]
pub struct DefaultTextStyle {
    pub brightness: f32,
    pub curve: f32,
    pub line_spacing: f64,
    pub top_drop: f64,
    pub font_size: f64,
    pub height_factor: f64,
}

impl Default for DefaultTextStyle {
    fn default() -> Self {
        Self {
            brightness: 1.0,
            curve: 0.5,
            line_spacing: 1.5,
            top_drop: 0.0,
            font_size: 9.0,
            height_factor: 0.0,
        }
    }
}
