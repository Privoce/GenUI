use makepad_widgets::{DVec2, Vec2};

pub trait AbsExt {
    fn is_in(&self, pos: &Vec2, size: &Vec2) -> bool;
}

impl AbsExt for DVec2 {
    fn is_in(&self, pos: &Vec2, size: &Vec2) -> bool {
        pos.x as f64 <= self.x
            && self.x <= pos.x as f64 + (size.x * 2.0) as f64
            && pos.y as f64 <= self.y
            && self.y <= pos.y as f64 + (size.y * 2.0) as f64
    }
}
