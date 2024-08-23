use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    DrawGLoading = {{DrawGLoading}}{

        fn pixel(self) -> vec4 {
            let loading_size = vec2(self.width, self.height);
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let loading_dot_size = vec2(loading_size.x * 0.2 * 0.96);
            let rotate_time = self.time;
            match self.loading_type{
                GLoadingType::Circle => {
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

                        sdf.circle(dot_pos.x, dot_pos.y, loading_dot_size.x * 0.5);
                        // with the time passing, the circle color(self.background_color) will change from deeper to lighter, then back to deeper
                        // It looks like it's spinning, but it's actually the color changing
                        // the easy way is to adjust the alpha value of the color
                        // let circle_color = self.background_color - vec4(0.0, 0.0, 0.0, 0.046 * counter);
                        sdf.fill(self.background_color * vec4(1.0, 1.0, 1.0, 0.5 + 0.5 * sin(rotate_time * 2 + counter * 0.1)));

                        counter += 1.0;
                    }
                }
                GLoadingType::DotLine => {
                    let r = loading_dot_size.x * 0.5;
                    let center = vec2(self.rect_size.x * 0.5, self.rect_size.y * 0.5);
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
                        sdf.fill(self.background_color);
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
    pub background_color: Vec4,
    #[live(64.0)]
    pub height: f32,
    #[live(64.0)]
    pub width: f32,
    #[live]
    pub loading_type: GLoadingType,
}

impl LiveHook for DrawGLoading {}

impl DrawGLoading {
    pub fn apply_loading_type(&mut self, loading_type: GLoadingType) {
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
}
