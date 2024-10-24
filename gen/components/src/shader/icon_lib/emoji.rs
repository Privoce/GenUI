use makepad_widgets::*;

use super::{types::emoji::Emoji, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconEmoji = {{DrawGIconEmoji}}{
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
                Emoji::Emoji => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.48;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x, center_y + quarter_size.y * 0.5, quarter_size.x);
                    sdf.rect(center_x - quarter_size.x, center_y - quarter_size.y * 0.5, quarter_size.x * 2.0, quarter_size.y);
                    sdf.subtract();
                    sdf.circle(center_x - quarter_size.x, center_y - quarter_size.y * 0.5, quarter_size.x * 0.5);
                    sdf.circle(center_x + quarter_size.x, center_y - quarter_size.y * 0.5, quarter_size.x * 0.5);
                    sdf.fill(self.stroke_color());
                }
                Emoji::Hot => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.6, center_y + quarter_size.y * 0.8);
                    let e = vec2(end_pos.x - quarter_size.x * 0.6, center_y + quarter_size.y * 0.8);
                    let c1 = vec2(center_x - quarter_size.x, end_pos.y);
                    let c2 = vec2(center_x + quarter_size.x, end_pos.y);
                    let counter1 = 0.0;
                    sdf.move_to(s.x, s.y);
                    // 绘制最下部分的火焰
                    for i in 0..100{
                        let point = self.bezier3(s, c1, c2, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    // 绘制左侧部分的火焰
                    let e2 = vec2(center_x - quarter_size.x * 0.5, start_pos.y);
                    let c3 = vec2(start_pos.x + quarter_size.x * 0.1, center_y - quarter_size.y * 0.4);
                    let c4 = vec2(center_x - quarter_size.x * 0.9, center_y - quarter_size.y * 0.2);
                    let counter2 = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier3(s, c3, c4, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    // 绘制右侧部分的火焰的左部分
                    let e3 = vec2(center_x + quarter_size.x * 0.6, center_y - quarter_size.y * 0.25);
                    let c5 = vec2(center_x + quarter_size.x * 0.2, start_pos.y + quarter_size.y * 0.35);
                    let counter3 = 0.0;
                    sdf.move_to(e2.x, e2.y);
                    for i in 0..100{
                        let point = self.bezier2(e2, c5, e3, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    let e4 = vec2(end_pos.x - quarter_size.x * 0.9, center_y - quarter_size.y * 0.9);
                    let c6 = vec2(end_pos.x - quarter_size.x * 1.2, center_y - quarter_size.y * 0.4);
                    let counter4 = 0.0;
                    sdf.move_to(e3.x, e3.y);
                    for i in 0..100{
                        let point = self.bezier2(e3, c6, e4, counter4 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter4 += 1.0;
                    }
                    let c7 = vec2(end_pos.x - quarter_size.x * 0.3, center_y + quarter_size.y * 0.1);
                    let counter5 = 0.0;
                    sdf.move_to(e4.x, e4.y);
                    for i in 0..100{
                        let point = self.bezier2(e4, c7, e, counter5 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter5 += 1.0;
                    }

                    sdf.stroke(self.stroke_color(), stroke_width);

                }
                Emoji::Heart => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e = vec2(end_pos.x - quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e1 = vec2(center_x, end_pos.y - quarter_size.y * 0.2);
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier2(s, c1, e1, counter / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter += 1.0;
                    }
                    let c2 = vec2(end_pos.x - quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter1 = 0.0;
                    sdf.move_to(e1.x, e1.y);
                    for i in 0..100{
                        let point = self.bezier2(e1, c2, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    
                    let c3 = vec2(end_pos.x - quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let e2 = vec2(center_x, center_y - quarter_size.y * 0.5);
                    let counter2 = 0.0;
                    sdf.move_to(e.x, e.y);
                    for i in 0..100{
                        let point = self.bezier2(e, c3, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    let c4 = vec2(start_pos.x + quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let counter3 = 0.0;
                    sdf.move_to(e2.x, e2.y);
                    for i in 0..100{
                        let point = self.bezier2(e2, c4, s, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Emoji::HeartBroken => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e = vec2(end_pos.x - quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e1 = vec2(center_x, end_pos.y - quarter_size.y * 0.2);
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier2(s, c1, e1, counter / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter += 1.0;
                    }
                    let c2 = vec2(end_pos.x - quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter1 = 0.0;
                    sdf.move_to(e1.x, e1.y);
                    for i in 0..100{
                        let point = self.bezier2(e1, c2, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    
                    let c3 = vec2(end_pos.x - quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let e2 = vec2(center_x, center_y - quarter_size.y * 0.5);
                    let counter2 = 0.0;
                    sdf.move_to(e.x, e.y);
                    for i in 0..100{
                        let point = self.bezier2(e, c3, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    let c4 = vec2(start_pos.x + quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let counter3 = 0.0;
                    sdf.move_to(e2.x, e2.y);
                    for i in 0..100{
                        let point = self.bezier2(e2, c4, s, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(e2.x, e2.y);
                    sdf.line_to(e2.x - quarter_size.x * 0.2, e2.y + quarter_size.y * 0.4);
                    sdf.line_to(e2.x + quarter_size.x * 0.5, e2.y + quarter_size.y * 0.66);
                    sdf.line_to(e2.x, e2.y + quarter_size.y * 1.0);
                    sdf.line_to(e2.x + quarter_size.x * 0.1, e2.y + quarter_size.y * 1.4);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Emoji::Dislike => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e = vec2(end_pos.x - quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e1 = vec2(center_x, end_pos.y - quarter_size.y * 0.2);
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier2(s, c1, e1, counter / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter += 1.0;
                    }
                    let c2 = vec2(end_pos.x - quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter1 = 0.0;
                    sdf.move_to(e1.x, e1.y);
                    for i in 0..100{
                        let point = self.bezier2(e1, c2, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    
                    let c3 = vec2(end_pos.x - quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let e2 = vec2(center_x, center_y - quarter_size.y * 0.5);
                    let counter2 = 0.0;
                    sdf.move_to(e.x, e.y);
                    for i in 0..100{
                        let point = self.bezier2(e, c3, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    let c4 = vec2(start_pos.x + quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let counter3 = 0.0;
                    sdf.move_to(e2.x, e2.y);
                    for i in 0..100{
                        let point = self.bezier2(e2, c4, s, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x - quarter_size.x * 0.5, center_y);
                    sdf.line_to(center_x + quarter_size.x * 0.5, end_pos.y - quarter_size.y * 1.1);
                    
                    sdf.move_to(center_x - quarter_size.x * 0.5, end_pos.y - quarter_size.y * 1.1);
                    sdf.line_to(center_x + quarter_size.x * 0.5, center_y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconEmoji {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: Emoji,
}

impl DrawGIconEmoji {
    pub fn apply_type(&mut self, ty: Emoji) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconEmoji {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}