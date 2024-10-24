use makepad_widgets::*;

use super::{types::person::Person, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconPerson = {{DrawGIconPerson}}{
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
                Person::Male => {
                    let quarter_size = size * 0.36;
                    sdf.circle(center_x - quarter_size.x * 0.4, center_y + quarter_size.y * 0.4, quarter_size.x);
                    sdf.move_to(center_x + quarter_size.x * 0.1, center_y - quarter_size.y * 0.1);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.2, start_pos.y + quarter_size.y * 0.2);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.8, start_pos.y + quarter_size.y * 0.2);
                    sdf.move_to(end_pos.x - quarter_size.x * 0.2, start_pos.y + quarter_size.y * 0.2);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.2, start_pos.y + quarter_size.y * 0.8);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Person::Female => {
                    let quarter_size = size * 0.36;
                    sdf.circle(center_x + quarter_size.x * 0.5, center_y - quarter_size.x * 0.5, quarter_size.x);
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.2, end_pos.y - quarter_size.y * 0.2);
                    
                    sdf.move_to(center_x - quarter_size.x * 1.0, center_y + quarter_size.y * 0.2);
                    sdf.line_to(center_x - quarter_size.x * 0.1, center_y + quarter_size.y * 1.0);
                    
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconPerson {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: Person,
}

impl DrawGIconPerson {
    pub fn apply_type(&mut self, ty: Person) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconPerson {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}