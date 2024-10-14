use makepad_widgets::*;

use super::{types::state::State, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconState = {{DrawGIconState}}{
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
                State::Info => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.5;
                    let r = stroke_width * 1.65;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x, center_y - quarter_size.y * 0.9 - r , r);
                    sdf.fill(self.stroke_color());
                    sdf.move_to(center_x - quarter_size.x * 0.25, center_y - quarter_size.y * 0.6);
                    sdf.line_to(center_x, center_y - quarter_size.y * 0.6);
                    sdf.line_to(center_x, center_y + quarter_size.y);
                    sdf.move_to(center_x - quarter_size.x * 0.4, center_y + quarter_size.y);
                    sdf.line_to(center_x + quarter_size.x * 0.4, center_y + quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width * 1.2);
                }
                State::Help => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.5;
                    let r = stroke_width * 1.65;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x, start_pos.x + quarter_size.y + half_size.y + r * 0.5, r);
                    sdf.fill(self.stroke_color());
                    let s = vec2(center_x - quarter_size.x * 0.6, center_y - quarter_size.y * 0.6);
                    let e1 = vec2(center_x + quarter_size.x * 0.6, center_y - quarter_size.y * 0.6);
                    let e2 = vec2(center_x, center_y);
                    let c1_1 = vec2(s.x, start_pos.y + quarter_size.y * 0.5);
                    let c1_2 = vec2(e1.x, start_pos.y + quarter_size.y * 0.5);
                    let c2 = vec2(center_x + quarter_size.x * 0.5, center_y - quarter_size.y * 0.1);
                    let counter1 = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier3(s, c1_1, c1_2 ,e1, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width * 1.2);
                    let counter2 = 0.0;
                    sdf.move_to(e1.x, e1.y);
                    for i in 0..100{
                        let point = self.bezier2(e1, c2, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width * 1.2);
                    sdf.move_to(e2.x, e2.y);
                    sdf.line_to(center_x, center_y + quarter_size.y * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width * 1.2);
                }
                State::Warn => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.5;
                    let r = stroke_width * 1.65;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.box(center_x - r * 0.8, start_pos.x + quarter_size.y * 0.5, r * 1.6, half_size.y, r * 0.25);
                    sdf.circle(center_x, start_pos.x + quarter_size.y + half_size.y + r * 0.5, r);
                    sdf.fill(self.stroke_color());
                }
                State::Wifi => {
                    let quarter_size = size * 0.25;
                    let spacing = quarter_size.x * 0.46;
                    let c1 = vec2(center_x, end_pos.y - spacing * 1.4);
                    let c2 = vec2(center_x, end_pos.y - spacing * 4.4);
                    let c3 = vec2(center_x, end_pos.y - spacing * 6.8);
                    let c4 = vec2(center_x, end_pos.y - spacing * 9.4);
                    let s2 = vec2(center_x - quarter_size.x * 0.8, end_pos.y - spacing * 2.8);
                    let e2 = vec2(center_x + quarter_size.x * 0.8, end_pos.y - spacing * 2.8);
                    let s3 = vec2(center_x - quarter_size.x * 1.4, end_pos.y - spacing * 4.0);
                    let e3 = vec2(center_x + quarter_size.x * 1.4, end_pos.y - spacing * 4.0);
                    let s4 = vec2(center_x - quarter_size.x * 2.2, end_pos.y - spacing * 5.4);
                    let e4 = vec2(center_x + quarter_size.x * 2.2, end_pos.y - spacing * 5.4);
                    sdf.circle(c1.x, c1.y, stroke_width * 2.0);
                    sdf.fill(self.stroke_color());
                    let counter2 = 0.0;
                    sdf.move_to(s2.x, s2.y);
                    for i in 0..100{
                        let point = self.bezier2(s2, c2, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let counter3 = 0.0;
                    sdf.move_to(s3.x, s3.y);
                    for i in 0..100{
                        let point = self.bezier2(s3, c3, e3, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let counter4 = 0.0;
                    sdf.move_to(s4.x, s4.y);
                    for i in 0..100{
                        let point = self.bezier2(s4, c4, e4, counter4 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter4 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                State::WifiNone => {
                    let quarter_size = size * 0.25;
                    let spacing = quarter_size.x * 0.46;
                    sdf.move_to(start_pos.x + quarter_size.x * 0.5, start_pos.y + quarter_size.y * 0.5);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.5, end_pos.y - quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let c1 = vec2(center_x, end_pos.y - spacing * 1.4);
                    let c2 = vec2(center_x, end_pos.y - spacing * 4.4);
                    let c3 = vec2(center_x, end_pos.y - spacing * 6.8);
                    let c4 = vec2(center_x, end_pos.y - spacing * 9.4);
                    let s2 = vec2(center_x - quarter_size.x * 0.8, end_pos.y - spacing * 2.8);
                    let e2 = vec2(center_x + quarter_size.x * 0.8, end_pos.y - spacing * 2.8);
                    let s3 = vec2(center_x - quarter_size.x * 1.4, end_pos.y - spacing * 4.0);
                    let e3 = vec2(center_x + quarter_size.x * 1.4, end_pos.y - spacing * 4.0);
                    let s4 = vec2(center_x - quarter_size.x * 2.2, end_pos.y - spacing * 5.4);
                    let e4 = vec2(center_x + quarter_size.x * 2.2, end_pos.y - spacing * 5.4);
                    sdf.circle(c1.x, c1.y, stroke_width * 2.0);
                    sdf.fill(self.stroke_color());
                    let counter2 = 0.0;
                    sdf.move_to(s2.x, s2.y);
                    for i in 0..100{
                        let point = self.bezier2(s2, c2, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let counter3 = 0.0;
                    sdf.move_to(s3.x, s3.y);
                    for i in 0..100{
                        let point = self.bezier2(s3, c3, e3, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let counter4 = 0.0;
                    sdf.move_to(s4.x, s4.y);
                    for i in 0..100{
                        let point = self.bezier2(s4, c4, e4, counter4 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter4 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconState {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: State,
}

impl DrawGIconState {
    pub fn apply_type(&mut self, ty: State) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconState {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}