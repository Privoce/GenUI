use makepad_widgets::*;

use super::{types::arrow::Arrow, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconArrow = {{DrawGIconArrow}}{
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
                Arrow::Left => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    // first draw left `<`
                    sdf.move_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(center_x - quarter_size.x, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Arrow::Right => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    // first draw left `<`
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(center_x + quarter_size.x, center_y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Arrow::Up => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(center_x, center_y - quarter_size.x);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Arrow::Down => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(center_x, center_y + quarter_size.x);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Arrow::Switch => {
                    // draw a `⇆` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
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
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
            }
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

impl DrawGIconArrow {
    pub fn apply_type(&mut self, ty: Arrow) {
        self.icon_type = ty;
    }
}
