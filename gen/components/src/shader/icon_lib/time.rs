use makepad_widgets::*;

use super::{types::time::Time, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconTime = {{DrawGIconTime}}{
        fn pixel(self) -> vec4{
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            // use offset to control not overlap with border
            let stroke_width = self.stroke_width;
            let offset = stroke_width * 1.25;
            let start_pos = vec2(self.pos.x + offset, self.pos.y + offset);
            let end_pos = vec2(self.pos.x + self.rect_size.x - offset * 1.0 - 1.0, self.pos.y + self.rect_size.y - offset * 1.0);
            let size = end_pos - start_pos;
            let center_y = self.rect_size.y * 0.5;
            let center_x = self.rect_size.x * 0.5;
            let half_size = size * 0.5;

            match self.icon_type{
                Time::Clock => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.5;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x, center_y - half_size.x * 0.8);
                    sdf.line_to(center_x, center_y);
                    sdf.line_to(center_x + half_size.x * 0.8, center_y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconTime {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: Time,
}

impl DrawGIconTime {
    pub fn apply_type(&mut self, ty: Time) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconTime {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}