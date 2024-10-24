use makepad_widgets::*;

use super::{types::relation::Relation, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconRelation = {{DrawGIconRelation}}{
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
                Relation::Connect => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 0.5;
                    sdf.circle(start_pos.x + quarter_size.x * 0.5, start_pos.y + quarter_size.y * 0.5, r);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y * 0.5);
                    sdf.line_to(center_x, start_pos.y + quarter_size.y * 0.5);
                    sdf.line_to(center_x, end_pos.y - quarter_size.y * 1.0);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.6, end_pos.y - quarter_size.y * 1.0);
                    sdf.line_to(end_pos.x - quarter_size.x * 1.2, end_pos.y - quarter_size.y * 1.5);
                    sdf.move_to(end_pos.x - quarter_size.x * 0.6, end_pos.y - quarter_size.y * 1.0);
                    sdf.line_to(end_pos.x - quarter_size.x * 1.2, end_pos.y - quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Relation::Disconnect => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 0.5;
                    sdf.circle(start_pos.x + quarter_size.x * 0.5, start_pos.y + quarter_size.y * 0.5, r);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y * 0.5);
                    sdf.line_to(center_x, start_pos.y + quarter_size.y * 0.5);
                    sdf.line_to(center_x, end_pos.y - quarter_size.y * 1.0);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.6, end_pos.y - quarter_size.y * 1.0);
                    sdf.line_to(end_pos.x - quarter_size.x * 1.2, end_pos.y - quarter_size.y * 1.5);
                    sdf.move_to(end_pos.x - quarter_size.x * 0.6, end_pos.y - quarter_size.y * 1.0);
                    sdf.line_to(end_pos.x - quarter_size.x * 1.2, end_pos.y - quarter_size.y * 0.5);
                    sdf.move_to(end_pos.x - quarter_size.x * 0.6, end_pos.y - quarter_size.y * 1.0);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.2, end_pos.y - quarter_size.y * 1.5);
                    sdf.move_to(end_pos.x - quarter_size.x * 0.6, end_pos.y - quarter_size.y * 1.0);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.2, end_pos.y - quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconRelation {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: Relation,
}

impl DrawGIconRelation {
    pub fn apply_type(&mut self, ty: Relation) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconRelation {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}