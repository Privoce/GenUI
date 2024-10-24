use makepad_widgets::*;

use super::{types::arrow::Arrow, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconArrow = {{DrawGIconArrow}}{
        fn pixel(self) -> vec4{
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            // use offset to control not overlap with border
            let stroke_width = self.stroke_width;
            let offset = stroke_width * 1.0;
            let start_pos = vec2(self.pos.x + offset, self.pos.y + offset);
            let end_pos = vec2(self.pos.x + self.rect_size.x - offset * 2.0, self.pos.y + self.rect_size.y - offset * 2.0);
            let size = end_pos - start_pos;
            let center_y = self.rect_size.y * 0.5;
            let center_x = self.rect_size.x * 0.5;
            let half_size = size * 0.5;
            let quarter_size = size * 0.25;
            match self.icon_type{
                Arrow::Left => {
                    // first draw left `<`
                    sdf.move_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y * 0.25);
                    sdf.line_to(center_x - quarter_size.x, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y * 0.25);
                }
                Arrow::Right => {
                    // first draw left `<`
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y * 0.25);
                    sdf.line_to(center_x + quarter_size.x, center_y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y  * 0.25);
                }
                Arrow::Up => {
                    sdf.move_to(start_pos.x + quarter_size.x * 0.25, end_pos.y - quarter_size.y);
                    sdf.line_to(center_x, center_y - quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.25, end_pos.y - quarter_size.y);
                }
                Arrow::Down => {
                    sdf.move_to(start_pos.x + quarter_size.x * 0.25, start_pos.y + quarter_size.y);
                    sdf.line_to(center_x, center_y + quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.25, start_pos.y + quarter_size.y);
                }
                Arrow::Switch => {
                    // draw a `⇆` icon as a button
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y);
                    sdf.line_to(start_pos.x, start_pos.y + half_size.y * 0.5);
                    sdf.line_to(end_pos.x, start_pos.y + half_size.y * 0.5);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x, start_pos.y + half_size.y * 0.5);
                    sdf.move_to(end_pos.x - quarter_size.x, end_pos.y);
                    sdf.line_to(end_pos.x, end_pos.y - half_size.y * 0.5);
                    sdf.line_to(start_pos.x, end_pos.y - half_size.y * 0.5);
                    sdf.move_to(end_pos.x - quarter_size.x, end_pos.y - half_size.y);
                    sdf.line_to(end_pos.x, end_pos.y - half_size.y * 0.5);
                }
            }
            sdf.stroke(self.stroke_color(), stroke_width);
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconArrow {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: Arrow,
}

// impl DrawGIconArrow {
//     pub fn apply_type(&mut self, ty: Arrow) {
//         self.icon_type = ty;
//     }
// }


impl ApplyIconType for DrawGIconArrow {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}