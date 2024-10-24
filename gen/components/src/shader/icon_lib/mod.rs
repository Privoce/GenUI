pub mod arrow;
pub mod base;
pub mod code;
pub mod emoji;
pub mod fs;
pub mod person;
pub mod relation;
pub mod state;
pub mod time;
pub mod tool;
pub mod types;
pub mod ui;

use std::error::Error;

use makepad_widgets::*;
use types::IconType;

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIcon = {{DrawGIcon}}{
        // draw bezier curve (2) http://wx.karlew.com/canvas/bezier/
        fn bezier2(self, start: vec2, control: vec2, end: vec2, t: float) -> vec2 {
            let u = 1.0 - t;
            let tt = t * t;
            let uu = u * u;
            let p = uu * start; // (1-t)^2 * p0
            p += 2.0 * u * t * control; // 2 * (1-t) * t * p1
            p += tt * end; // t^2 * p2
            return p;
        }
        // draw bezier curve (3)
        fn bezier3(self, start: vec2, control1: vec2, control2: vec2, end: vec2, t: float) -> vec2 {
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

        // uv = self.pos * self.rect_size
        fn circle(self, uv: vec2, center: vec2, radius: float, color: vec4) -> vec4 {
            let dist = length(uv - center);
            let edge = smoothstep(radius - 0.01, radius + 0.01, dist);
            return mix(color, vec4(0.0), edge);
        }

        // draw arc
        fn arc_circle(self, uv: vec2, x: float, y: float, r: float, s: float, e: float, color: vec4) -> vec4 {
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

        fn gear(self, uv: vec2, center: vec2, size: float, color: vec4) -> vec4 {
            let pi = 3.141592653589793;
            let num_teeth = 6.0;
            let angle_step = 2.0 * pi / float(num_teeth);
            let small_circle_radius = size * 0.125;
            let large_circle_radius = size * 0.5;
            let inner_circle_radius = size * 0.2;
            let counter = 0.0;

            // 绘制大圆
            let result = self.circle(uv, center, large_circle_radius, color);
            // 绘制一个大圆的同心小圆，这个小圆中的部分是透明的
            let inner_circle = self.circle(uv, center, inner_circle_radius, vec4(1.0));
            result = result * (vec4(1.0) - inner_circle);


            // 绘制齿轮的齿
            for i in 0..6 {
                let angle = counter * angle_step;
                let tooth_center = center + vec2(cos(angle), sin(angle)) * large_circle_radius;
                let tooth = self.circle(uv, tooth_center, small_circle_radius, vec4(1.0));
                result = result * (vec4(1.0) - tooth);
                counter += 1.0;
            }

            return result;
        }

        fn stroke_color(self) -> vec4 {
            return mix(
                mix(
                    self.color,
                    self.stroke_hover_color,
                    self.hover
                ),
                self.stroke_focus_color,
                self.focus
            );
        }

        fn pixel(self) -> vec4{
            return self.stroke_color();
        }
    }
}

/// super draw struct for icon lib
#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIcon {
    #[deref]
    pub deref_draw: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live]
    pub stroke_hover_color: Vec4, 
    #[live]
    pub border_color: Vec4,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub stroke_width: f32,
    #[live(0.0)]
    pub hover: f32,
    #[live(0.0)]
    pub focus: f32,
    #[live]
    pub stroke_focus_color: Vec4,
}

pub trait ApplyIconType {
    fn apply_type(&mut self, ty: &IconType) -> Result<(), Box<dyn Error>>;
}