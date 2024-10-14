use makepad_widgets::*;

use super::{types::code::Code, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconCode = {{DrawGIconCode}}{
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
                Code::Code => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    // first draw left `<`
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    // then draw right `>`
                    sdf.move_to(end_pos.x - quarter_size.y, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.move_to(end_pos.x - quarter_size.y * 1.25, start_pos.y + quarter_size.y * 0.25);
                    sdf.line_to(start_pos.x + quarter_size.x * 1.25, end_pos.y - quarter_size.y * 0.25);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Code::Test => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.76, end_pos.y - quarter_size.y * 1.1);
                    let e = vec2(start_pos.x + quarter_size.x * 1.64, end_pos.y - quarter_size.y * 0.5);
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.6, end_pos.y - quarter_size.y * 0.1);
                    sdf.move_to(center_x + quarter_size.x * 0.4, start_pos.y + quarter_size.y * 0.36);
                    sdf.line_to(s.x, s.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    let counter = 0.0;
                    for i in 0..100{
                        let point = self.bezier2(s, c1, e, counter / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.line_to(center_x + quarter_size.x * 0.8 * 1.68, start_pos.y + quarter_size.y * 0.84);
                    sdf.line_to(center_x + quarter_size.x * 0.4, start_pos.y + quarter_size.y * 0.36);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x - quarter_size.x * 0.14, center_y - quarter_size.y * 0.6);
                    sdf.line_to(center_x + quarter_size.x * 0.9, center_y - quarter_size.y * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let s2 = vec2(center_x + quarter_size.x * 1.6, center_y + quarter_size.y * 0.46);
                    let e2 = vec2(center_x + quarter_size.x * 1.6, center_y + quarter_size.y * 1.3);
                    let c2 = vec2(center_x +quarter_size.x * 1.0, center_y + quarter_size.y * 1.22);
                    let c3 = vec2(center_x + quarter_size.x * 2.2, center_y + quarter_size.y * 1.22);
                    let counter2 = 0.0;
                    sdf.move_to(s2.x, s2.y);
                    for i in 0..100{
                        let point = self.bezier2(s2, c2, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s2.x, s2.y);
                    let counter3 = 0.0;
                    for i in 0..100{
                        let point = self.bezier2(s2, c3, e2, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    
                }
                Code::Debug => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 1.0;
                    let c = vec2(center_x, start_pos.y + r * 0.9);
                    // draw head
                    sdf.circle(c.x, c.y, r);
                    sdf.rect(c.x - r, c.y, r * 2.0, r);
                    sdf.subtract();
                    sdf.fill(self.stroke_color());
                    // draw body
                    let s1 = vec2(start_pos.x + quarter_size.x * 0.6, start_pos.y + r * 1.1);
                    let s2 = vec2(end_pos.x - quarter_size.x * 0.6, start_pos.y + r * 1.1);
                    sdf.move_to(s1.x, s1.y);
                    sdf.line_to(s2.x, s2.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let e = vec2(center_x, end_pos.y - quarter_size.y * 0.5);
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.25, end_pos.y - quarter_size.y * 0.5);
                    let c2 = vec2(end_pos.x - quarter_size.x * 0.25, end_pos.y - quarter_size.y * 0.5);
                    let counter1 = 0.0;
                    sdf.move_to(s1.x, s1.y);
                    for i in 0..100{
                        let point = self.bezier2(s1, c1, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    let counter2 = 0.0;
                    sdf.move_to(s2.x, s2.y);
                    for i in 0..100{
                        let point = self.bezier2(s2, c2, e, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(e.x, e.y);
                    sdf.line_to(e.x, center_y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    // draw legs
                    let offset_q = quarter_size.x * 0.707106;
                    let s3 = vec2(s1.x - offset_q, s1.y - offset_q);
                    let s4 = vec2(s2.x + offset_q, s2.y - offset_q);
                    let e3 = vec2(s2.x + offset_q, e.y);
                    let e4 = vec2(s1.x - offset_q, e.y);
                    sdf.move_to(s3.x, s3.y);
                    sdf.line_to(s1.x, s1.y);
                    sdf.move_to(s3.x, center_y);
                    sdf.line_to(s1.x, center_y);
                    sdf.move_to(s4.x, center_y);
                    sdf.line_to(s2.x, center_y);
                    sdf.move_to(s4.x, s4.y);
                    sdf.line_to(s2.x, s2.y);
                    sdf.move_to(e3.x, e3.y);
                    sdf.line_to(s2.x, c2.y - (e3.x - s2.x));
                    sdf.move_to(e4.x, e4.y);
                    sdf.line_to(s1.x, c1.y - (s1.x - e4.x));
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconCode {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: Code,
}

impl DrawGIconCode {
    pub fn apply_type(&mut self, ty: Code) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconCode {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}