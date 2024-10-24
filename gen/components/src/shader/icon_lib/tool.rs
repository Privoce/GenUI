use makepad_widgets::*;

use super::{types::tool::Tool, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconTool = {{DrawGIconTool}}{
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
                Tool::Search => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 1.25;
                    sdf.circle(center_x, center_y, r);
                    sdf.move_to(center_x + r * 0.707106, center_y + r * 0.707106);
                    sdf.line_to(end_pos.x - quarter_size.y * 0.4, end_pos.y - quarter_size.y * 0.4);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let s = vec2(center_x - quarter_size.x * 0.5, center_y - quarter_size.y * 0.5);
                    let e = vec2(center_x + quarter_size.x * 0.5, center_y - quarter_size.y * 0.5);
                    let c1 = vec2(center_x, center_y - quarter_size.y * 1.0);
                    let counter = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier2(s, c1, e, counter / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Tool::ZoomIn => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 1.25;
                    sdf.circle(center_x, center_y, r);
                    sdf.move_to(center_x + r * 0.707106, center_y + r * 0.707106);
                    sdf.line_to(end_pos.x - quarter_size.y * 0.4, end_pos.y - quarter_size.y * 0.4);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x - quarter_size.x * 0.5, center_y);
                    sdf.line_to(center_x + quarter_size.x * 0.5, center_y);
                    sdf.move_to(center_x, center_y - quarter_size.y * 0.5);
                    sdf.line_to(center_x, center_y + quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Tool::ZoomOut => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 1.25;
                    sdf.circle(center_x, center_y, r);
                    sdf.move_to(center_x + r * 0.707106, center_y + r * 0.707106);
                    sdf.line_to(end_pos.x - quarter_size.y * 0.4, end_pos.y - quarter_size.y * 0.4);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x - quarter_size.x * 0.5, center_y);
                    sdf.line_to(center_x + quarter_size.x * 0.5, center_y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Tool::Share => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 0.4;
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.6, center_y);
                    let c2 = vec2(end_pos.x - quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let c3 = vec2(end_pos.x - quarter_size.x * 0.6, end_pos.y - quarter_size.y * 0.6);
                    sdf.circle(c1.x, c1.y, r);
                    sdf.move_to(c1.x, c1.y);
                    sdf.line_to(c2.x, c2.y);
                    sdf.circle(c2.x, c2.y, r);
                    sdf.move_to(c1.x, c1.y);
                    sdf.line_to(c3.x, c3.y);
                    sdf.circle(c3.x, c3.y, r);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    
                }
                Tool::Rss => {
                    let quarter_size = size * 0.25;
                    sdf.move_to(center_x, center_y + quarter_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.6, end_pos.y - quarter_size.y * 0.4);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.4);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.4);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.6, end_pos.y - quarter_size.y * 0.4);
                    sdf.close_path();
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x, center_y - quarter_size.y * 0.6);
                    sdf.line_to(center_x, center_y + quarter_size.y * 0.6);
                    sdf.move_to(center_x - quarter_size.x * 0.6, center_y);
                    sdf.line_to(center_x + quarter_size.x * 0.6, center_y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Tool::AI => {
                    let quarter_size = size * 0.25;
                    let h = half_size.y * 2.0 - quarter_size.y * 1.6;
                    let w = half_size.x * 2.0 - quarter_size.x;
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.66, start_pos.y + quarter_size.y * 0.4);
                    let c2 = vec2(end_pos.x - quarter_size.x * 0.66, start_pos.y + quarter_size.y * 0.4);
                    let r = quarter_size.x * 0.2;
                    let s = vec2(start_pos.x + quarter_size.x * 0.5, start_pos.y + h / 2.0);
                    sdf.box(s.x, s.y, w, h, 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(c1.x, c1.y, r);
                    sdf.circle(c2.x, c2.y, r);
                    sdf.fill(self.stroke_color());
                    sdf.move_to(c1.x + r, c1.y + r);
                    sdf.line_to(center_x - quarter_size.x * 0.6, start_pos.y + h / 2.0);
                    sdf.move_to(c2.x - r, c2.y + r);
                    sdf.line_to(center_x + quarter_size.x * 0.6, start_pos.y + h / 2.0);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, center_y);
                    sdf.line_to(s.x - quarter_size.x * 0.25, center_y);
                    sdf.line_to(s.x - quarter_size.x * 0.25, s.y + h - quarter_size.y * 0.5);
                    sdf.line_to(s.x, s.y + h - quarter_size.y * 0.5);
                    sdf.move_to(s.x + w, center_y);
                    sdf.line_to(s.x + w + quarter_size.x * 0.25, center_y);
                    sdf.line_to(s.x + w + quarter_size.x * 0.25, s.y + h - quarter_size.y * 0.5);
                    sdf.line_to(s.x + w, s.y + h - quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.box(center_x - w * 0.325, center_y, w * 0.65, h * 0.2, h * 0.2 * 0.25);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Tool::VR => {
                    let quarter_size = size * 0.25;
                    let h = half_size.y * 2.0 - quarter_size.y * 2.0;
                    let w = half_size.x * 2.0 - quarter_size.x;
                    let r = quarter_size.x * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.5, start_pos.y + h / 2.0);
                    let c1 = vec2(s.x + w * 0.25, center_y);
                    let c2 = vec2(s.x + w - w * 0.25, center_y);
                    sdf.box(s.x, s.y, w, h, 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(c1.x, c1.y, r);
                    sdf.circle(c2.x, c2.y, r);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, start_pos.y + quarter_size.y * 0.6);
                    sdf.line_to(s.x + w, start_pos.y + quarter_size.y * 0.6);
                    sdf.move_to(s.x, end_pos.y - quarter_size.y * 0.6);
                    sdf.line_to(s.x + w, end_pos.y - quarter_size.y * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                
                Tool::Notice => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 0.2;
                    let s = vec2(center_x, start_pos.y + quarter_size.y * 0.4 + r * 0.5);
                    let e1 = vec2(start_pos.x + quarter_size.x * 0.5, end_pos.y - quarter_size.y * 0.8);
                    let e2 = vec2(end_pos.x - quarter_size.x * 0.5, end_pos.y - quarter_size.y * 0.8);
                    let c1_1 = vec2(center_x - quarter_size.x * 0.25, center_y + quarter_size.y * 0.2);
                    let c1_2 = vec2(center_x + quarter_size.x * 0.25, center_y + quarter_size.y * 0.2);
                    let c2 = vec2(start_pos.x, start_pos.y + quarter_size.y * 0.6);
                    let c3 = vec2(end_pos.x, start_pos.y + quarter_size.y * 0.6);
                    sdf.circle(s.x, start_pos.y + quarter_size.y * 0.4, r);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    let counter1 = 0.0;
                    for i in 0..100{
                        let point = self.bezier3(s, c2, c1_1, e1, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    let counter2 = 0.0;
                    for i in 0..100{
                        let point = self.bezier3(s, c3, c1_2, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.move_to(e1.x, e1.y);
                    sdf.line_to(e2.x, e2.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x, e1.y + quarter_size.y * 0.1);
                    sdf.line_to(center_x, e1.y + quarter_size.y * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width * 1.5);
                }
                Tool::NoticeNone => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 0.2;
                    let s = vec2(center_x, start_pos.y + quarter_size.y * 0.4 + r * 0.5);
                    let e1 = vec2(start_pos.x + quarter_size.x * 0.5, end_pos.y - quarter_size.y * 0.8);
                    let e2 = vec2(end_pos.x - quarter_size.x * 0.5, end_pos.y - quarter_size.y * 0.8);
                    let c1_1 = vec2(center_x - quarter_size.x * 0.25, center_y + quarter_size.y * 0.2);
                    let c1_2 = vec2(center_x + quarter_size.x * 0.25, center_y + quarter_size.y * 0.2);
                    let c2 = vec2(start_pos.x, start_pos.y + quarter_size.y * 0.6);
                    let c3 = vec2(end_pos.x, start_pos.y + quarter_size.y * 0.6);
                    sdf.move_to(start_pos.x + quarter_size.x * 0.5, start_pos.y + quarter_size.y * 0.5);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.5, end_pos.y - quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(s.x, start_pos.y + quarter_size.y * 0.4, r);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    let counter1 = 0.0;
                    for i in 0..100{
                        let point = self.bezier3(s, c2, c1_1, e1, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    let counter2 = 0.0;
                    for i in 0..100{
                        let point = self.bezier3(s, c3, c1_2, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.move_to(e1.x, e1.y);
                    sdf.line_to(e2.x, e2.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x, e1.y + quarter_size.y * 0.1);
                    sdf.line_to(center_x, e1.y + quarter_size.y * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width * 1.5);
                }
                Tool::Bind => {
                    // draw a 📌 icon
                    let quarter_size = size * 0.25;
                    let q_q_size = quarter_size * 0.7;
                    sdf.move_to(center_x - q_q_size.x, start_pos.y + stroke_width);
                    sdf.line_to(center_x + q_q_size.x, start_pos.y + stroke_width);
                    sdf.line_to(center_x + q_q_size.x * 0.66, center_y - q_q_size.y * 1.86);
                    sdf.line_to(center_x + quarter_size.x * 1.15, center_y);
                    sdf.line_to(center_x - quarter_size.x * 1.15, center_y);
                    sdf.line_to(center_x - q_q_size.x * 0.66, center_y - q_q_size.y * 1.86);
                    sdf.close_path();
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(center_x, end_pos.y - q_q_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconTool {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: Tool,
}

impl DrawGIconTool {
    pub fn apply_type(&mut self, ty: Tool) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconTool {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}