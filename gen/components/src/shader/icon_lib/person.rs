use makepad_widgets::*;

use super::{types::person::Person, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconPerson = {{DrawGIconPerson}}{
        fn pixel(self) -> vec4{
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            // use offset to control not overlap with border
            let stroke_width = 1.2;
            let offset = stroke_width * 1.25;
            let start_pos = vec2(self.pos.x - self.border_width + offset, self.pos.y - self.border_width + offset);
            let end_pos = vec2(self.pos.x + self.rect_size.x - self.border_width - offset * 1.0 - 1.0, self.pos.y + self.rect_size.y - self.border_width - offset * 1.0);
            let size = end_pos - start_pos;
            let center_y = self.rect_size.y * 0.5;
            let center_x = self.rect_size.x * 0.5;
            let half_size = size * 0.5;

            match self.icon_type{
                Person::Male => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.fill(self.stroke_color());
                }
                Person::Female => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.fill(self.stroke_color());
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
