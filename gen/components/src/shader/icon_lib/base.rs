use makepad_widgets::*;

use super::{
    types::{base::Base, IconType},
    ApplyIconType, DrawGIcon,
};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconBase = {{DrawGIconBase}}{
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
                Base::Min => {
                    // draw a `-` icon as a button
                    let quarter_size = size * 0.25;
                    let h = stroke_width * 3.5;
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y - h / 2.0, size.x - quarter_size.x / 2.0, h, 0.6);
                    sdf.fill(self.stroke_color());
                }
                Base::Max => {
                    // draw a `▢` icon as a button
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x + quarter_size.x / 2.0, start_pos.y + quarter_size.x / 2.0, size.x - quarter_size.x, size.y- quarter_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::FullScreen => {
                    // draw a `▣` icon as a button
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x + quarter_size.x / 2.0, start_pos.y + quarter_size.x / 2.0, size.x - quarter_size.x, size.y- quarter_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let inner_size = size * 0.45;
                    sdf.rect(start_pos.x + size.x * 0.275, start_pos.y + size.y * 0.275, inner_size.x, inner_size.y);
                    sdf.fill(self.stroke_color());
                }
                Base::FullScreenExpand => {
                    let quarter_size = size * 0.3;
                    sdf.move_to(start_pos.x + quarter_size.x * 0.2, start_pos.y + quarter_size.x);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.2, start_pos.y + quarter_size.x * 0.2);
                    sdf.line_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.x * 0.2);
                    // ------------------------------
                    sdf.move_to(start_pos.x + quarter_size.x * 0.2, end_pos.y - quarter_size.x);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.2, end_pos.y - quarter_size.x * 0.2);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.x * 0.2);
                    // ------------------------------
                    sdf.move_to(end_pos.x - quarter_size.x * 0.2, end_pos.y - quarter_size.x );
                    sdf.line_to(end_pos.x - quarter_size.x * 0.2, end_pos.y - quarter_size.x * 0.2);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.x * 0.2);
                    // ------------------------------
                    sdf.move_to(end_pos.x - quarter_size.x * 0.2, start_pos.y + quarter_size.x);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.2, start_pos.y + quarter_size.x * 0.2);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.x * 0.2);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.rect(start_pos.x + quarter_size.x, start_pos.y + quarter_size.x, size.x - quarter_size.x * 2.0, size.y- quarter_size.x * 2.0);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::More => {
                    // draw a `⋯` icon as a button
                    sdf.circle(start_pos.x , center_y, stroke_width * 1.5);
                    sdf.circle(center_x - stroke_width * 0.75, center_y, stroke_width * 1.5);
                    sdf.circle(end_pos.x - stroke_width, center_y, stroke_width * 1.5);
                    sdf.fill(self.stroke_color());
                }
                Base::Close => {
                    // draw a `×` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::Upload =>{
                    let quarter_size = size * 0.25;
                    let h = size.y * 0.8;
                    let w = size.x;
                    let r_c = size.x * 0.2;
                    sdf.box(start_pos.x, start_pos.y, w, h, stroke_width);
                    sdf.move_to(center_x, start_pos.y + h);
                    sdf.line_to(center_x, end_pos.y);
                    sdf.move_to(center_x - quarter_size.x, end_pos.y);
                    sdf.line_to(center_x + quarter_size.x, end_pos.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(end_pos.x - r_c * 2.0, start_pos.y + r_c);
                    sdf.line_to(end_pos.x - r_c * 1.5, start_pos.y + r_c * 0.5);
                    sdf.line_to(end_pos.x - r_c, start_pos.y + r_c);
                    sdf.move_to(end_pos.x - r_c * 1.5, start_pos.y + r_c * 0.5);
                    sdf.line_to(end_pos.x - r_c * 1.5, start_pos.y + r_c * 2.0);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::Download => {
                    let quarter_size = size * 0.25;
                    let h = size.y * 0.8;
                    let w = size.x;
                    let r_c = size.x * 0.2;
                    sdf.box(start_pos.x, start_pos.y, w, h, stroke_width);
                    sdf.move_to(center_x, start_pos.y + h);
                    sdf.line_to(center_x, end_pos.y);
                    sdf.move_to(center_x - quarter_size.x, end_pos.y);
                    sdf.line_to(center_x + quarter_size.x, end_pos.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(end_pos.x - r_c * 2.0, start_pos.y + r_c * 1.5);
                    sdf.line_to(end_pos.x - r_c * 1.5, start_pos.y + r_c * 2.0);
                    sdf.line_to(end_pos.x - r_c, start_pos.y + r_c * 1.5);
                    sdf.move_to(end_pos.x - r_c * 1.5, start_pos.y + r_c * 0.5);
                    sdf.line_to(end_pos.x - r_c * 1.5, start_pos.y + r_c * 2.0);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::Add => {
                    let quarter_size = size * 0.15;
                    sdf.move_to(center_x, start_pos.y);
                    sdf.line_to(center_x, end_pos.y);
                    sdf.move_to(start_pos.x, center_y);
                    sdf.line_to(end_pos.x, center_y);

                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::Delete => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    let q_q_size = quarter_size * 0.8;
                    //line ----------------------------------------------------
                    sdf.move_to(start_pos.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + quarter_size.y);
                    //---------------------------------------------------------
                    sdf.move_to(start_pos.x + q_q_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x + q_q_size.x, start_pos.y + quarter_size.y - q_q_size.y);
                    sdf.line_to(end_pos.x - q_q_size.x, start_pos.y + quarter_size.y - q_q_size.y);
                    sdf.line_to(end_pos.x - q_q_size.x, start_pos.y + quarter_size.y);
                    sdf.move_to(start_pos.x + q_q_size.x * 0.6, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x + q_q_size.x, end_pos.y);
                    sdf.line_to(end_pos.x - q_q_size.x, end_pos.y);
                    sdf.line_to(end_pos.x - q_q_size.x * 0.6, start_pos.y + quarter_size.y);
                    sdf.move_to(start_pos.x + quarter_size.x * 1.5, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x * 1.5, end_pos.y - quarter_size.y)
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::DeleteKey => {
                    let half_size = size * 0.5;
                    let inner_size = size * 0.4;
                    let quarter_size = size * 0.25;
                    let q_q_size = quarter_size * 0.8;
                    let offset_w = size.x * 0.1;
                    // draw a `⌫` icon as a button
                    // first draw outer
                    sdf.move_to(start_pos.x, center_y);
                    sdf.line_to(start_pos.x + q_q_size.x * 1.1, start_pos.y + q_q_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + q_q_size.y);
                    sdf.line_to(end_pos.x, end_pos.y - q_q_size.y);
                    sdf.line_to(start_pos.x + q_q_size.x * 1.1, end_pos.y - q_q_size.y);
                    sdf.line_to(start_pos.x, center_y);
                    // then draw a `×` icon
                    sdf.move_to(start_pos.x + inner_size.x + offset_w, start_pos.y + inner_size.y);
                    sdf.line_to(end_pos.x - inner_size.x + offset_w, end_pos.y - inner_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x + inner_size.x + offset_w, end_pos.y - inner_size.y);
                    sdf.line_to(end_pos.x - inner_size.x + offset_w, start_pos.y + inner_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);

                }
                Base::Correct => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.2;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::Fresh => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    // use bezier curve to draw a circle with arrow, but the circle is not perfect
                    // let the circle lack of a quarter
                    sdf.circle(center_x + stroke_width, center_y, half_size.x* 0.95);
                    sdf.stroke(self.arc_circle(self.pos * self.rect_size, center_x, center_y, half_size.x * 0.95, 0.0, 1.0, self.stroke_color()), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x * 1.0, start_pos.y);
                    sdf.line_to(start_pos.x + (center_x * 1.1 - sin(0.25 * 3.14159265358979) * center_x), start_pos.y + center_y - cos(0.25 * 3.14159265358979) * center_y);
                    sdf.line_to(start_pos.x + quarter_size.x * 1.68, start_pos.y + (center_y - cos(0.25 * 3.14159265358979) * center_y) * 1.86);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::Play => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.48;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    // draw a `▶` icon
                    sdf.move_to((center_x - quarter_size.x) * 1.414, center_y - quarter_size.y);
                    sdf.line_to(center_x + quarter_size.x, center_y);
                    sdf.line_to((center_x - quarter_size.x) * 1.414, center_y + quarter_size.y);
                    sdf.line_to((center_x - quarter_size.x) * 1.414, center_y - quarter_size.y);
                    // sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.fill(self.stroke_color());
                }
                Base::Stop => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.48;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.rect(center_x - quarter_size.x * 1.2, center_y - quarter_size.y, quarter_size.x * 0.8, quarter_size.y * 2.0);
                    sdf.rect(center_x + quarter_size.x * 0.48, center_y - quarter_size.y, quarter_size.x * 0.8, quarter_size.y * 2.0);
                    sdf.fill(self.stroke_color());
                }
                Base::GoOn => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.48;
                    let r = half_size.x;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.rect(center_x - r / 2.0, center_y - r / 2.0, r , r);
                    sdf.fill(self.stroke_color());
                }
                Base::Setting => {
                    // draw a `⚙` icon as a button, svg path:
                    let r = size.x * 0.5;
                    let quarter_size = size * 0.25;
                    let col = self.gear(self.pos * self.rect_size, vec2(center_x, center_y), size.x, self.stroke_color());
                    sdf.circle(center_x, center_y, r);
                    sdf.fill(col);
                }
                Base::Setting2 => {
                    let r =  half_size.x;
                    sdf.hexagon(center_x, center_y, r);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x, center_y, r * 0.4);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::Setting3 => {
                    let quarter_size = size * 0.25;
                    let h = stroke_width * 2.0;
                    let h2 = h * 2.4;
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y - h / 2.0 * 6.0, size.x - quarter_size.x / 4.0, h, 0.6);
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y, size.x - quarter_size.x / 4.0, h, 0.6);
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y + h / 2.0 * 6.0, size.x - quarter_size.x / 4.0, h, 0.6);
                    sdf.fill(self.stroke_color());
                    sdf.rect(center_x + quarter_size.x * 0.8, center_y - h / 2.0 * 6.0 - h2 / 3.0, h, h2);
                    sdf.rect(center_x - quarter_size.x * 0.8, center_y - h2 / 3.0, h, h2);
                    sdf.rect(center_x + quarter_size.x * 0.8, center_y + h / 2.0 * 6.0 - h2 / 3.0, h, h2);
                    sdf.fill(self.stroke_color());
                }
                Base::Home => {
                    let quarter_size = size.x * 0.08;
                    let w = size.x * 0.4;
                    sdf.move_to(center_x, start_pos.y + quarter_size);
                    sdf.line_to(start_pos.x + quarter_size, center_y- quarter_size);
                    sdf.line_to(start_pos.x + quarter_size, end_pos.y - quarter_size);
                    sdf.line_to(center_x - w / 2.0, end_pos.y - quarter_size);
                    sdf.line_to(center_x - w / 2.0, end_pos.y - w);
                    sdf.line_to(center_x + w / 2.0, end_pos.y - w);
                    sdf.line_to(center_x + w / 2.0, end_pos.y - quarter_size);
                    sdf.line_to(end_pos.x - quarter_size, end_pos.y - quarter_size);
                    sdf.line_to(end_pos.x - quarter_size, center_y - quarter_size);
                    sdf.close_path();
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.rect(center_x - quarter_size, center_y + w / 2.0, quarter_size * 2.0, center_y * 0.5);
                    sdf.fill(self.stroke_color());
                }
                Base::System => {
                    let quarter_size = size * 0.25;
                    let r = quarter_size.x * 0.65;
                    sdf.rect(start_pos.x + quarter_size.x * 0.2, start_pos.y + quarter_size.y * 0.2, r * 2.0, r * 2.0);
                    sdf.rect(start_pos.x + quarter_size.x * 0.2, end_pos.y - quarter_size.y * 0.2 - r *2.0, r* 2.0, r* 2.0);
                    sdf.rect(end_pos.x - quarter_size.x * 0.2 - r * 2.0, end_pos.y - quarter_size.y * 0.2 - r * 2.0, r* 2.0, r* 2.0);
                    sdf.circle(end_pos.x - quarter_size.x * 0.2 - r, start_pos.y + quarter_size.y * 0.2 + r, r);
                    sdf.stroke(self.stroke_color(), stroke_width);

                }
                Base::Picture => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x + quarter_size.x * 0.2, start_pos.y + quarter_size.x * 0.2, size.x - quarter_size.x * 0.2, size.y- quarter_size.x * 0.2);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x * 0.2, center_y + quarter_size.y * 1.0);
                    sdf.line_to(center_x - quarter_size.x * 0.9, center_y + quarter_size.y * 0.45);
                    sdf.line_to(center_x - quarter_size.x * 0.2, center_y + quarter_size.y * 0.9);
                    sdf.line_to(center_x + quarter_size.x * 0.9, center_y + quarter_size.y * 0.1);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.2, center_y + quarter_size.y * 0.8);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x - quarter_size.x * 0.2, center_y - quarter_size.y * 0.5, quarter_size.x * 0.46);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::Eye => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.6, center_y);
                    let e = vec2(end_pos.x - quarter_size.x * 0.6, center_y);
                    let c1 = vec2(center_x, start_pos.y + quarter_size.y * 0.2);
                    let c2 = vec2(center_x, end_pos.y - quarter_size.y * 0.2);
                    let counter1 = 0.0;
                    let counter2 = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier2(s, c1, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier2(s, c2, e, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x, center_y, quarter_size.x * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::EyeClose => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.6, center_y);
                    let e = vec2(end_pos.x - quarter_size.x * 0.6, center_y);
                    let c1 = vec2(center_x, start_pos.y + quarter_size.y * 0.2);
                    let c2 = vec2(center_x, end_pos.y - quarter_size.y * 0.2);
                    let counter1 = 0.0;
                    let counter2 = 0.0;
                    sdf.move_to(center_x - quarter_size.x, start_pos.y + quarter_size.y * 0.8);
                    sdf.line_to(center_x + quarter_size.x, end_pos.y - quarter_size.y * 0.8);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier2(s, c1, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = self.bezier2(s, c2, e, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x, center_y, quarter_size.x * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }

                Base::Phone => {
                    let quarter_size = size * 0.25;
                    let w = size.x - quarter_size.x;
                    let h = size.y - quarter_size.x / 4.0;
                    let e = start_pos.y + h;
                    sdf.rect(center_x - w / 2.0, start_pos.y, w, h);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.rect(center_x - w / 4.0, e - stroke_width * 5.0, w / 2.0, stroke_width * 2.0);
                    sdf.fill(self.stroke_color());
                }
                Base::Light => {
                    let quarter_size = size * 0.25;
                    sdf.move_to(center_x - quarter_size.x * 0.55, start_pos.y + quarter_size.y * 0.2);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.4, center_y + quarter_size.y * 0.2);
                    sdf.line_to(center_x - quarter_size.x * 0.3, center_y + quarter_size.y * 0.2);
                    sdf.line_to(center_x - quarter_size.x * 0.45, end_pos.y - quarter_size.y * 0.25);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.4, center_y - quarter_size.y * 0.25);
                    sdf.line_to(center_x + quarter_size.x * 0.35, center_y - quarter_size.y * 0.25);
                    sdf.line_to(center_x + quarter_size.x * 1.35, start_pos.y + quarter_size.y * 0.2);
                    sdf.close_path();
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                Base::Menu => {
                    let quarter_size = size * 0.25;
                    let h = stroke_width * 2.0;
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y - h * 4.0, size.x - quarter_size.x / 2.0, h, 0.6);
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y - h , size.x - quarter_size.x / 2.0, h, 0.6);
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y + h * 2.0, size.x - quarter_size.x / 2.0, h, 0.6);
                    sdf.fill(self.stroke_color());
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconBase {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: Base,
}

impl DrawGIconBase {
    pub fn apply_type(&mut self, ty: Base) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconBase {
    fn apply_type(&mut self, ty: &IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}
