use makepad_widgets::*;

use super::{types::fs::Fs, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconFs = {{DrawGIconFs}}{
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
                Fs::Note => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.4, start_pos.y + quarter_size.y * 0.2);
                    let e = vec2(end_pos.x - quarter_size.x * 0.4, end_pos.y - quarter_size.y * 0.2);
                    sdf.move_to(s.x, s.y);
                    sdf.line_to(s.x, e.y);
                    sdf.line_to(e.x, e.y);
                    sdf.line_to(e.x, s.y + quarter_size.y * 0.8);
                    sdf.line_to(e.x - quarter_size.x * 0.8, s.y);
                    sdf.close_path();
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x + quarter_size.x * 0.5, center_y - quarter_size.y * 0.5);
                    sdf.line_to(e.x - quarter_size.x * 0.5, center_y - quarter_size.y * 0.5);
                    sdf.move_to(s.x + quarter_size.x * 0.5, center_y + quarter_size.y * 0.5);
                    sdf.line_to(e.x - quarter_size.x * 0.5, center_y + quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), quarter_size.y * 0.2);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconFs {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: Fs,
}

impl DrawGIconFs {
    pub fn apply_type(&mut self, ty: Fs) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconFs {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}