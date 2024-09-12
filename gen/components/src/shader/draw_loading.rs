use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    DrawGLoading = {{DrawGLoading}}{
        fn loading_circle(self, color: vec4) -> vec4 {
            let pi = 3.141592653589793;
            let uv = self.pos * self.rect_size;
            let center = self.rect_size * 0.5;
            let aspect = self.rect_size.x / self.rect_size.y;
            
            // 将UV坐标调整为以中心为原点，并考虑宽高比
            let adjusted_uv = (uv - center) / vec2(aspect, 1.0);
            
            let radius = min(self.rect_size.x, self.rect_size.y) * 0.4;
            let line_width = min(self.rect_size.x, self.rect_size.y) * 0.03;
            let glow_size = line_width * 3.0;
            
            let len = length(adjusted_uv);
            let angle = atan(adjusted_uv.y, adjusted_uv.x);
            
            // 计算旋转和渐变效果
            let rotation_speed = 0.5;
            let fall_off = fract(-0.5 * (angle / pi) - self.time * rotation_speed);
            
            // 计算圆环的形状
            let circle_shape = smoothstep(line_width, 0.0, abs(radius - len));
            
            // 添加发光效果
            let glow = smoothstep(glow_size * fall_off, 0.0, abs(radius - len) - line_width * 0.5) * 0.5;
            
            // 组合形状和发光效果
            let shape = (circle_shape + glow) * fall_off;
            
            // 创建颜色渐变
            let gradient_color = mix(vec4(color.rgb, 0.1), color, fall_off);
            
            return gradient_color * shape;
        }

        fn pixel(self) -> vec4 {
            let loading_size =  self.rect_size * 0.86;
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let loading_dot_size = vec2(loading_size.x * 0.2 * 0.96);
            let rotate_time = self.time;
            let center = vec2(self.rect_size.x * 0.5, self.rect_size.y * 0.5);
            match self.loading_type{
                GLoadingType::Circle => {
                    return self.loading_circle(self.stroke_color);
                }
                GLoadingType::DotLine => {
                    let r = loading_dot_size.x * 0.5;

                    let spacing = (loading_size.x - 8.0 * r) * 0.25 + 2.0 * r;
                    // let phase = (rotate_time / 2.0 - rotate_time) / 2.0;
                    let num_dots = 5;
                    let counter = 0.0;
                    for i in 0..5 {
                        // let t = counter / 4;
                        // let offset = abs(phase - t) * loading_size.x * 0.5;
                        let offset = abs(2.0 - counter) * spacing ;
                        if counter < 2.0 {
                            let dot_pos = vec2(center.x + offset * sin(rotate_time), center.y);

                            sdf.circle(dot_pos.x, dot_pos.y, r);
                        }else{
                            let dot_pos = vec2(center.x - offset * sin(rotate_time), center.y);

                            sdf.circle(dot_pos.x, dot_pos.y, r);
                        }
                        sdf.fill(self.stroke_color);
                        counter += 1.0;
                    }
                }
                GLoadingType::CircleDot => {
                    let counter = 0.0;
                    // draw 16 dots around as a loading animation
                    for i in 0..16{
                        // each dot is a circle and we place it around the circle, with a bit of spacing
                        // there are 16 dots so angle is 0.125PI
                        let angle = 0.125 * 3.1415926;
                        let dot_pos = vec2(
                            self.rect_size.x * 0.5 - cos(angle * counter) * loading_size.x * 0.5,
                            self.rect_size.y * 0.5 - sin(angle * counter) * loading_size.y * 0.5
                        );

                        sdf.circle(dot_pos.x, dot_pos.y, loading_dot_size.x * 0.4 + 0.1 * counter);
                        // with the time passing, the circle color(self.stroke_color) will change from deeper to lighter, then back to deeper
                        // It looks like it's spinning, but it's actually the color changing
                        // the easy way is to adjust the alpha value of the color
                        // let circle_color = self.stroke_color - vec4(0.0, 0.0, 0.0, 0.046 * counter);
                        sdf.fill(self.stroke_color * vec4(1.0, 1.0, 1.0, 0.5 + 0.5 * sin(rotate_time * 2 + counter * 0.1)));

                        counter += 1.0;
                    }
                }
            }

            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister)]
#[repr(C)]
pub struct DrawGLoading {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub stroke_color: Vec4,
    #[live]
    pub loading_type: GLoadingType,
}

impl LiveHook for DrawGLoading {}

impl DrawGLoading {
    pub fn apply_type(&mut self, loading_type: GLoadingType) {
        self.loading_type = loading_type;
    }
}

#[derive(Live, LiveHook, Clone)]
#[live_ignore]
#[repr(u32)]
pub enum GLoadingType {
    #[pick]
    Circle = shader_enum(1),
    DotLine = shader_enum(2),
    CircleDot = shader_enum(3),
}
