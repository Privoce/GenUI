use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;

    DrawGToolButton = {{DrawGToolButton}}{
        // draw bezier curve (2) http://wx.karlew.com/canvas/bezier/
        fn bezier2(start: vec2, control: vec2, end: vec2, t: float) -> vec2 {
            let u = 1.0 - t;
            let tt = t * t;
            let uu = u * u;
            let p = uu * start; // (1-t)^2 * p0
            p += 2.0 * u * t * control; // 2 * (1-t) * t * p1
            p += tt * end; // t^2 * p2
            return p;
        }
        // draw bezier curve (3)
        fn bezier3(start: vec2, control1: vec2, control2: vec2, end: vec2, t: float) -> vec2 {
            let u = 1.0 - t;
            let tt = t * t;
            let uu = u * u;
            let uuu = uu * u;
            let ttt = tt * t;
            let p = uuu * start; // (1-t)^3 * p0
            p += 3.0 * uu * t * control1; // 3 * (1-t)^2 * t * p1
            p += 3.0 * u * tt * control2; // 3 * (1-t) * t^2 * p2
            p += ttt * end; // t^3 * p3
            return p;
        }
        // 绘制二次贝塞尔曲线并返回颜色值
        fn draw_bezier2(start: vec2, control: vec2, end: vec2, color: vec4, thickness: float) -> vec4 {
            let point = bezier2(start, control, end, thickness);
            return vec4(point, 0.0, 1.0);
        }

        // 绘制三次贝塞尔曲线并返回颜色值
        fn draw_bezier3(uv: vec2, start: vec2, control1: vec2, control2: vec2, end: vec2, color: vec4, thickness: float) -> vec4 {

        }
        fn circle(uv: vec2, center: vec2, radius: float, color: vec4) -> vec4 {
            let dist = length(uv - center);
            let edge = smoothstep(radius - 0.01, radius + 0.01, dist);
            return mix(color, vec4(0.0), edge);
        }

        fn gear(uv: vec2, center: vec2, size: float, color: vec4) -> vec4 {
            let pi = 3.141592653589793;
            let num_teeth = 6.0;
            let angle_step = 2.0 * pi / float(num_teeth);
            let small_circle_radius = size * 0.125;
            let large_circle_radius = size * 0.5;
            let inner_circle_radius = size * 0.2;
            let counter = 0.0;

            // 绘制大圆
            let result = circle(uv, center, large_circle_radius, color);
            // 绘制一个大圆的同心小圆，这个小圆中的部分是透明的
            let inner_circle = circle(uv, center, inner_circle_radius, vec4(1.0));
            result = result * (vec4(1.0) - inner_circle);


            // 绘制齿轮的齿
            for i in 0..6 {
                let angle = counter * angle_step;
                let tooth_center = center + vec2(cos(angle), sin(angle)) * large_circle_radius;
                let tooth = circle(uv, tooth_center, small_circle_radius, vec4(1.0));
                result = result * (vec4(1.0) - tooth);
                counter += 1.0;
            }

            return result;
        }
        // draw arc
        fn arc_circle(uv: vec2, x: float, y: float, r: float, s: float, e: float, color: vec4) -> vec4 {
            let c = uv - vec2(x, y);
            let pi = 3.141592653589793; // PI constant

            // Calculate angle in range [0, 1]
            let ang = (atan(c.y, c.x) + pi) / (2.0 * pi);

            // Normalize start and end angles to range [0, 1]
            let s_norm = s / (2.0 * pi);
            let e_norm = e / (2.0 * pi);

            // Check if angle is within the arc range
            let in_arc = step(s_norm, ang) * step(ang, e_norm);

            // Calculate distance from the center
            let dist = length(c) - r;

            // Mix color based on distance and arc range
            let color_factor = smoothstep(-0.01, 0.01, -dist) * in_arc;

            return mix(color, vec4(0.0), color_factor);
        }

        fn stroke_color(self) -> vec4 {
            return mix(
                self.stroke_color,
                self.hover_color,
                self.hover
            );
        }

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

            match self.tool_button_type {
                GToolButtonType::Min => {
                    // draw a `-` icon as a button
                    let quarter_size = size * 0.25;
                    let h = stroke_width * 3.5;
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y - h / 2.0, size.x - quarter_size.x / 2.0, h, 0.6);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Max => {
                    // draw a `▢` icon as a button
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x + quarter_size.x / 2.0, start_pos.y + quarter_size.x / 2.0, size.x - quarter_size.x, size.y- quarter_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::FullScreen => {
                    // draw a `▣` icon as a button
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x + quarter_size.x / 2.0, start_pos.y + quarter_size.x / 2.0, size.x - quarter_size.x, size.y- quarter_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let inner_size = size * 0.45;
                    sdf.rect(start_pos.x + size.x * 0.275, start_pos.y + size.y * 0.275, inner_size.x, inner_size.y);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::FullScreenExpand => {
                    let quarter_size = size * 0.3;
                    sdf.move_to(start_pos.x, start_pos.y + quarter_size.x);
                    sdf.line_to(start_pos.x, start_pos.y);
                    sdf.line_to(start_pos.x + quarter_size.x, start_pos.y);
                    // ------------------------------
                    sdf.move_to(start_pos.x, end_pos.y - quarter_size.x);
                    sdf.line_to(start_pos.x, end_pos.y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y);
                    // ------------------------------
                    sdf.move_to(end_pos.x, end_pos.y - quarter_size.x);
                    sdf.line_to(end_pos.x, end_pos.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y);
                    // ------------------------------
                    sdf.move_to(end_pos.x, start_pos.y + quarter_size.x);
                    sdf.line_to(end_pos.x, start_pos.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.rect(start_pos.x + quarter_size.x, start_pos.y + quarter_size.x, size.x - quarter_size.x * 2.0, size.y- quarter_size.x * 2.0);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Left => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    // first draw left `<`
                    sdf.move_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(center_x - quarter_size.x, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Right => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    // first draw left `<`
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(center_x + quarter_size.x, center_y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::More => {
                    // draw a `⋯` icon as a button
                    sdf.circle(start_pos.x , center_y, stroke_width * 1.5);
                    sdf.circle(center_x, center_y, stroke_width * 1.5);
                    sdf.circle(end_pos.x - stroke_width, center_y, stroke_width * 1.5);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Close => {
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
                GToolButtonType::Up => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(center_x, center_y - quarter_size.x);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Down => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(center_x, center_y + quarter_size.x);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Switch => {
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
                GToolButtonType::Exit => {
                    let offset_smooth = 0.7;
                    sdf.move_to(end_pos.x * offset_smooth, start_pos.y);
                    sdf.line_to(start_pos.x, start_pos.y);
                    sdf.line_to(start_pos.x, end_pos.y);
                    sdf.line_to(end_pos.x * offset_smooth, end_pos.y);
                    sdf.move_to(end_pos.x - end_pos.x * (1.0 - offset_smooth) + size.x * 0.1, self.rect_size.y * 0.5 - size.y * 0.3);
                    sdf.line_to(end_pos.x , self.rect_size.y * 0.5);
                    sdf.line_to(end_pos.x - end_pos.x * (1.0 - offset_smooth) + size.x * 0.1, self.rect_size.y * 0.5 + size.y * 0.3);
                    sdf.move_to(end_pos.x, self.rect_size.y * 0.5);
                    sdf.line_to(end_pos.x - size.x * 0.5, self.rect_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Expand => {
                    // draw a `<>` icon as a button
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
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::ExpandTop => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::ExpandBottom => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::ExpandLeft => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.y, start_pos.y);
                    sdf.line_to(start_pos.x + quarter_size.y, end_pos.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::ExpandRight => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(end_pos.x - quarter_size.y, start_pos.y);
                    sdf.line_to(end_pos.x - quarter_size.y, end_pos.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Upload =>{
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
                GToolButtonType::Download => {
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
                GToolButtonType::Add => {
                    let quarter_size = size * 0.15;
                    sdf.move_to(center_x, start_pos.y + quarter_size.y);
                    sdf.line_to(center_x, end_pos.y - quarter_size.y);
                    sdf.move_to(start_pos.x + quarter_size.x, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x, center_y);

                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Delete => {
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
                GToolButtonType::DeleteKey => {
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
                GToolButtonType::Correct => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.2;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Fresh => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    // use bezier curve to draw a circle with arrow, but the circle is not perfect
                    // let the circle lack of a quarter
                    sdf.circle(center_x + stroke_width, center_y, half_size.x* 0.95);
                    sdf.stroke(arc_circle(self.pos * self.rect_size, center_x, center_y, half_size.x * 0.95, 0.0, 1.0, self.stroke_color()), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x * 1.0, start_pos.y);
                    sdf.line_to(start_pos.x + (center_x * 1.1 - sin(0.25 * 3.14159265358979) * center_x), start_pos.y + center_y - cos(0.25 * 3.14159265358979) * center_y);
                    sdf.line_to(start_pos.x + quarter_size.x * 1.68, start_pos.y + (center_y - cos(0.25 * 3.14159265358979) * center_y) * 1.86);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Play => {
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
                GToolButtonType::Stop => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.48;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.rect(center_x - quarter_size.x * 1.2, center_y - quarter_size.y, quarter_size.x * 0.8, quarter_size.y * 2.0);
                    sdf.rect(center_x + quarter_size.x * 0.48, center_y - quarter_size.y, quarter_size.x * 0.8, quarter_size.y * 2.0);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Setting => {
                    // draw a `⚙` icon as a button, svg path:
                    let r = size.x * 0.5;
                    let quarter_size = size * 0.25;
                    let col = gear(self.pos * self.rect_size, vec2(center_x, center_y), size.x, self.stroke_color());
                    sdf.circle(center_x, center_y, r);
                    sdf.fill(col);
                }
                GToolButtonType::Bind => {
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
                GToolButtonType::Menu => {
                    let quarter_size = size * 0.25;
                    let h = stroke_width * 4.0;
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y - h / 2.0 * 4.0, size.x - quarter_size.x / 2.0, h, 0.6);
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y - h / 2.0, size.x - quarter_size.x / 2.0, h, 0.6);
                    sdf.box(start_pos.x + quarter_size.x / 4.0, center_y + h * 1.0, size.x - quarter_size.x / 2.0, h, 0.6);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Emoji => {
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
                GToolButtonType::Phone => {
                    let quarter_size = size * 0.25;
                    let w = size.x - quarter_size.x;
                    let h = size.y - quarter_size.x / 4.0;
                    let e = start_pos.y + h;
                    sdf.rect(center_x - w / 2.0, start_pos.y, w, h);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.rect(center_x - w / 4.0, e - stroke_width * 5.0, w / 2.0, stroke_width * 2.0);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Default => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::GoOn => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.48;
                    let r = half_size.x;
                    sdf.circle(center_x, center_y, half_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.rect(center_x - r / 2.0, center_y - r / 2.0, r , r);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Setting2 => {
                    let r =  half_size.x;
                    sdf.hexagon(center_x, center_y, r);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x, center_y, r * 0.4);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Hot => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.6, center_y + quarter_size.y * 0.8);
                    let e = vec2(end_pos.x - quarter_size.x * 0.6, center_y + quarter_size.y * 0.8);
                    let c1 = vec2(center_x - quarter_size.x, end_pos.y);
                    let c2 = vec2(center_x + quarter_size.x, end_pos.y);
                    let counter1 = 0.0;
                    sdf.move_to(s.x, s.y);
                    // 绘制最下部分的火焰
                    for i in 0..100{
                        let point = bezier3(s, c1, c2, e, counter1 / 100.0);
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
                        let point = bezier3(s, c3, c4, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    // 绘制右侧部分的火焰的左部分
                    let e3 = vec2(center_x + quarter_size.x * 0.6, center_y - quarter_size.y * 0.25);
                    let c5 = vec2(center_x + quarter_size.x * 0.2, start_pos.y + quarter_size.y * 0.35);
                    let counter3 = 0.0;
                    sdf.move_to(e2.x, e2.y);
                    for i in 0..100{
                        let point = bezier2(e2, c5, e3, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    let e4 = vec2(end_pos.x - quarter_size.x * 0.9, center_y - quarter_size.y * 0.9);
                    let c6 = vec2(end_pos.x - quarter_size.x * 1.2, center_y - quarter_size.y * 0.4);
                    let counter4 = 0.0;
                    sdf.move_to(e3.x, e3.y);
                    for i in 0..100{
                        let point = bezier2(e3, c6, e4, counter4 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter4 += 1.0;
                    }
                    let c7 = vec2(end_pos.x - quarter_size.x * 0.3, center_y + quarter_size.y * 0.1);
                    let counter5 = 0.0;
                    sdf.move_to(e4.x, e4.y);
                    for i in 0..100{
                        let point = bezier2(e4, c7, e, counter5 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter5 += 1.0;
                    }

                    sdf.stroke(self.stroke_color(), stroke_width);

                }
                GToolButtonType::Heart => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e = vec2(end_pos.x - quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e1 = vec2(center_x, end_pos.y - quarter_size.y * 0.2);
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = bezier2(s, c1, e1, counter / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter += 1.0;
                    }
                    let c2 = vec2(end_pos.x - quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter1 = 0.0;
                    sdf.move_to(e1.x, e1.y);
                    for i in 0..100{
                        let point = bezier2(e1, c2, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    
                    let c3 = vec2(end_pos.x - quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let e2 = vec2(center_x, center_y - quarter_size.y * 0.5);
                    let counter2 = 0.0;
                    sdf.move_to(e.x, e.y);
                    for i in 0..100{
                        let point = bezier2(e, c3, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    let c4 = vec2(start_pos.x + quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let counter3 = 0.0;
                    sdf.move_to(e2.x, e2.y);
                    for i in 0..100{
                        let point = bezier2(e2, c4, s, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::HeartBroken => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e = vec2(end_pos.x - quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e1 = vec2(center_x, end_pos.y - quarter_size.y * 0.2);
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = bezier2(s, c1, e1, counter / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter += 1.0;
                    }
                    let c2 = vec2(end_pos.x - quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter1 = 0.0;
                    sdf.move_to(e1.x, e1.y);
                    for i in 0..100{
                        let point = bezier2(e1, c2, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    
                    let c3 = vec2(end_pos.x - quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let e2 = vec2(center_x, center_y - quarter_size.y * 0.5);
                    let counter2 = 0.0;
                    sdf.move_to(e.x, e.y);
                    for i in 0..100{
                        let point = bezier2(e, c3, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    let c4 = vec2(start_pos.x + quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let counter3 = 0.0;
                    sdf.move_to(e2.x, e2.y);
                    for i in 0..100{
                        let point = bezier2(e2, c4, s, counter3 / 100.0);
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
                GToolButtonType::Dislike => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e = vec2(end_pos.x - quarter_size.x * 0.3, center_y - quarter_size.y * 0.1);
                    let e1 = vec2(center_x, end_pos.y - quarter_size.y * 0.2);
                    let c1 = vec2(start_pos.x + quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = bezier2(s, c1, e1, counter / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter += 1.0;
                    }
                    let c2 = vec2(end_pos.x - quarter_size.x * 0.1, center_y + quarter_size.y * 0.86);
                    let counter1 = 0.0;
                    sdf.move_to(e1.x, e1.y);
                    for i in 0..100{
                        let point = bezier2(e1, c2, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    
                    let c3 = vec2(end_pos.x - quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let e2 = vec2(center_x, center_y - quarter_size.y * 0.5);
                    let counter2 = 0.0;
                    sdf.move_to(e.x, e.y);
                    for i in 0..100{
                        let point = bezier2(e, c3, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    let c4 = vec2(start_pos.x + quarter_size.x * 0.6, start_pos.y + quarter_size.y * 0.6);
                    let counter3 = 0.0;
                    sdf.move_to(e2.x, e2.y);
                    for i in 0..100{
                        let point = bezier2(e2, c4, s, counter3 / 100.0);
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
                GToolButtonType::Rss => {
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
                GToolButtonType::Share => {
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
                GToolButtonType::ZoomIn => {
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
                GToolButtonType::ZoomOut => {
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
                GToolButtonType::Eye => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.6, center_y);
                    let e = vec2(end_pos.x - quarter_size.x * 0.6, center_y);
                    let c1 = vec2(center_x, start_pos.y + quarter_size.y * 0.2);
                    let c2 = vec2(center_x, end_pos.y - quarter_size.y * 0.2);
                    let counter1 = 0.0;
                    let counter2 = 0.0;
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = bezier2(s, c1, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = bezier2(s, c2, e, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x, center_y, quarter_size.x * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::EyeClose => {
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
                        let point = bezier2(s, c1, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    for i in 0..100{
                        let point = bezier2(s, c2, e, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.circle(center_x, center_y, quarter_size.x * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Search => {
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
                        let point = bezier2(s, c1, e, counter / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Connect => {
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
                GToolButtonType::Disconnect => {
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
                GToolButtonType::Debug => {
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
                        let point = bezier2(s1, c1, e, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    let counter2 = 0.0;
                    sdf.move_to(s2.x, s2.y);
                    for i in 0..100{
                        let point = bezier2(s2, c2, e, counter2 / 100.0);
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
                GToolButtonType::Code => {
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
                GToolButtonType::Test => {
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
                        let point = bezier2(s, c1, e, counter / 100.0);
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
                        let point = bezier2(s2, c2, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s2.x, s2.y);
                    let counter3 = 0.0;
                    for i in 0..100{
                        let point = bezier2(s2, c3, e2, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    
                }
                GToolButtonType::Open => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x, center_y, quarter_size.x);
                    sdf.move_to(center_x - quarter_size.x, center_y);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.1, center_y);
                    sdf.move_to(center_x + quarter_size.x, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.1, center_y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::OpenLeft => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x + quarter_size.x * 0.4, center_y, quarter_size.x);
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.2, center_y);
                    sdf.line_to(center_x - quarter_size.x * 0.8, center_y - quarter_size.y * 0.6);
                    sdf.move_to(start_pos.x + quarter_size.x * 0.2, center_y);
                    sdf.line_to(center_x - quarter_size.x * 0.8, center_y + quarter_size.y * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::OpenRight => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x - quarter_size.x * 0.4, center_y, quarter_size.x);
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.2, center_y);
                    sdf.line_to(center_x + quarter_size.x * 0.8, center_y - quarter_size.y * 0.6);
                    sdf.move_to(end_pos.x - quarter_size.x * 0.2, center_y);
                    sdf.line_to(center_x + quarter_size.x * 0.8, center_y + quarter_size.y * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::OpenTop => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x, center_y + quarter_size.y * 0.4, quarter_size.x);
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(center_x, start_pos.y + quarter_size.y * 0.2);
                    sdf.line_to(center_x - quarter_size.x * 0.6, center_y - quarter_size.y * 0.8);
                    sdf.move_to(center_x, start_pos.y + quarter_size.y * 0.2);
                    sdf.line_to(center_x + quarter_size.x * 0.6, center_y - quarter_size.y * 0.8);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::OpenBottom => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x, center_y - quarter_size.y * 0.4, quarter_size.x);
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(center_x, end_pos.y - quarter_size.y * 0.2);
                    sdf.line_to(center_x - quarter_size.x * 0.6, center_y + quarter_size.y * 0.8);
                    sdf.move_to(center_x, end_pos.y - quarter_size.y * 0.2);
                    sdf.line_to(center_x + quarter_size.x * 0.6, center_y + quarter_size.y * 0.8);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Split => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x + quarter_size.x / 2.0, start_pos.y + quarter_size.x / 2.0, size.x - quarter_size.x, size.y- quarter_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x, start_pos.y + quarter_size.y * 0.5);
                    sdf.line_to(center_x, end_pos.y - quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Split2 => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x + quarter_size.x / 2.0, start_pos.y + quarter_size.x / 2.0, size.x - quarter_size.x, size.y- quarter_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x * 0.5, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.5, center_y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Wifi => {
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
                        let point = bezier2(s2, c2, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let counter3 = 0.0;
                    sdf.move_to(s3.x, s3.y);
                    for i in 0..100{
                        let point = bezier2(s3, c3, e3, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let counter4 = 0.0;
                    sdf.move_to(s4.x, s4.y);
                    for i in 0..100{
                        let point = bezier2(s4, c4, e4, counter4 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter4 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::WifiNone => {
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
                        let point = bezier2(s2, c2, e2, counter2 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter2 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let counter3 = 0.0;
                    sdf.move_to(s3.x, s3.y);
                    for i in 0..100{
                        let point = bezier2(s3, c3, e3, counter3 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter3 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    let counter4 = 0.0;
                    sdf.move_to(s4.x, s4.y);
                    for i in 0..100{
                        let point = bezier2(s4, c4, e4, counter4 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter4 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::AI => {
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
                GToolButtonType::VR => {
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
                GToolButtonType::Note => {
                    let quarter_size = size * 0.25;
                    let s = vec2(start_pos.x + quarter_size.x * 0.4, start_pos.y + quarter_size.y * 0.2);
                    let e = vec2(end_pos.x - quarter_size.x * 0.4, end_pos.y - quarter_size.y * 0.2);
                    sdf.move_to(s.x, s.y);
                    sdf.line_to(s.x, e.y);
                    sdf.line_to(e.x, e.y);
                    sdf.line_to(e.x, s.y + quarter_size.y * 0.8);
                    sdf.line_to(e.x - quarter_size.x * 0.8, s.y);
                    sdf.close_path();
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x + quarter_size.x * 0.5, center_y - quarter_size.y * 0.5);
                    sdf.line_to(e.x - quarter_size.x * 0.5, center_y - quarter_size.y * 0.5);
                    sdf.move_to(s.x + quarter_size.x * 0.5, center_y + quarter_size.y * 0.5);
                    sdf.line_to(e.x - quarter_size.x * 0.5, center_y + quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), quarter_size.y * 0.2);
                }
                GToolButtonType::Notice => {
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
                        let point = bezier3(s, c2, c1_1, e1, counter1 / 100.0);
                        sdf.line_to(point.x, point.y);
                        counter1 += 1.0;
                    }
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(s.x, s.y);
                    let counter2 = 0.0;
                    for i in 0..100{
                        let point = bezier3(s, c3, c1_2, e2, counter2 / 100.0);
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
                GToolButtonType::NoticeNone => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Clock => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Info => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Help => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Warn => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Poweroff => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                GToolButtonType::Light => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Male => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Female => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Setting3 => {
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
                GToolButtonType::Picture => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::System => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.fill(self.stroke_color());
                }
                GToolButtonType::Home => {
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
            }
            return sdf.result;
        }

    }
}

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum GToolButtonType {
    /// `-` (减号, 缩小, 最小化)
    Min = shader_enum(1),
    /// `▢` (最大化)
    Max = shader_enum(2),
    /// `▣` (全屏)
    FullScreen = shader_enum(3),
    /// `<` (左箭头)
    Left = shader_enum(4),
    /// `>` (右箭头)
    Right = shader_enum(5),
    /// `⋯` (更多)
    More = shader_enum(6),
    /// `×` (关闭)
    Close = shader_enum(7),
    /// `︿` (向上)
    Up = shader_enum(8),
    /// `﹀` (向下)
    Down = shader_enum(9),
    /// `⇆` (切换)
    Switch = shader_enum(10),
    /// ```
    /// ---
    /// |  > (退出)
    /// ---
    /// ```
    Exit = shader_enum(11),
    /// `<>` (展开)
    Expand = shader_enum(12),
    /// ```
    /// -------
    /// |-----|  (展开上边)
    /// |     |
    /// -------
    /// ```
    ExpandTop = shader_enum(13),
    /// ```
    /// -------
    /// |     |
    /// |-----|  (展开下边)
    /// -------
    /// ```
    ExpandBottom = shader_enum(14),
    /// ```
    /// ---------
    /// |  |    |  (展开左边)
    /// |  |    |
    /// ---------
    /// ```
    ExpandLeft = shader_enum(15),
    /// ```
    /// ---------
    /// |    |  |  (展开右边)
    /// |    |  |
    /// ---------
    ExpandRight = shader_enum(16),
    /// 上面有一朵云下面有个向上的箭头
    /// a cloud with an arrow pointing up below
    Upload = shader_enum(17),
    /// 上面有一朵云下面有个向下的箭头
    /// a cloud with an arrow pointing down below
    Download = shader_enum(18),
    /// `+` (加号)
    Add = shader_enum(19),
    /// 一个垃圾桶
    /// a trash can
    Delete = shader_enum(20),
    /// `✓` (勾)
    Correct = shader_enum(21),
    /// `↺` (刷新)
    Fresh = shader_enum(22),
    /// 一个圆其中有一个 ▶ (播放)
    /// a circle with a ▶ (play)
    Play = shader_enum(23),
    /// 一个圆其中有一个斜向下的横线 (停止)
    /// a circle with a diagonal line down (stop)
    Stop = shader_enum(24),
    /// 一个设置图标
    Setting = shader_enum(25),
    /// 一个类似📌图标
    /// a similar 📌 icon
    Bind = shader_enum(26),
    /// `≡` (菜单)
    Menu = shader_enum(27),
    /// 一个笑脸, 类似😀
    /// a smiley face, similar to 😀
    Emoji = shader_enum(28),
    /// 一个电话, 类似📱
    /// a phone, similar to 📱
    Phone = shader_enum(29),
    #[pick]
    Default = shader_enum(30),
    DeleteKey = shader_enum(31),
    FullScreenExpand,
    Setting2,
    Setting3,
    Hot,
    Heart,
    HeartBroken,
    Dislike,
    Rss,
    Share,
    ZoomIn,
    ZoomOut,
    Eye,
    EyeClose,
    Search,
    Connect,
    Disconnect,
    Debug,
    Code,
    Test,
    Open,
    OpenLeft,
    OpenRight,
    OpenTop,
    OpenBottom,
    Split,
    Split2,
    Wifi,
    WifiNone,
    AI,
    VR,
    Note,
    Notice,
    NoticeNone,
    Clock,
    /// i
    Info,
    /// ?
    Help,
    /// !
    Warn,
    Poweroff,
    Light,
    Male,
    Female,
    Home,
    System,
    Picture,
    GoOn,
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGToolButton {
    #[deref]
    pub draw_super: DrawQuad,
    #[live(0.0_f32)]
    pub hover: f32,
    #[live]
    pub stroke_color: Vec4,
    #[live]
    pub hover_color: Vec4,
    #[live(0.0_f32)]
    pub border_width: f32,
    #[live]
    pub border_color: Vec4,
    #[live]
    pub tool_button_type: GToolButtonType,
}

impl DrawGToolButton {
    pub fn apply_button_type(&mut self, tool_button_type: GToolButtonType) {
        self.tool_button_type = tool_button_type;
    }
}
