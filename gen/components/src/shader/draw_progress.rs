use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawGProgress = {{DrawGProgress}}{

        fn get_background_color(self) -> vec4 {
            return mix(
                mix(
                    self.background_color,
                    self.hover_color,
                    self.hover
                ),
                self.focus_color,
                self.focus
            );
        }
        fn get_stroke_color(self) -> vec4 {
            return mix(
                mix(
                    self.stroke_color,
                    self.stroke_hover_color,
                    self.hover
                ),
                self.stroke_focus_color,
                self.focus
            );
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            // let progress_height = self.rect_size.y - 2.0 * self.border_width;
            // let progress_width = self.rect_size.x - 2.0 * self.border_width;
            let progress_height = self.rect_size.y - self.border_width * 2.0;
            let progress_width = self.rect_size.x - self.border_width * 2.0;
            let progress_bg = self.get_background_color();
            let progress_in_bg = self.get_stroke_color();
            sdf.box(self.pos.x, self.pos.y, progress_width, progress_height, self.border_radius);
            if self.background_visible == 1.0{
                sdf.fill(progress_bg);
            }
            sdf.stroke(self.border_color, self.border_width);
            match self.progress_type {
                GProgressType::Horizontal => {
                    let box_radius = self.border_radius - self.border_width * 0.25;
                    sdf.box(
                            self.pos.x,
                            self.pos.y,
                            self.position * self.rect_size.x - self.border_width * 2.0,
                            progress_height,
                            box_radius
                    )
                    sdf.fill(progress_in_bg);
                }
                GProgressType::Vertical => {
                    let box_radius = self.border_radius - self.border_width * 0.25;
                    // pos should be end of progress
                    sdf.box(
                        self.pos.x,
                        self.rect_size.y - self.rect_size.y * self.position ,
                        self.rect_size.x - self.border_width * 2.0,
                        self.rect_size.y * self.position - self.border_width * 1.0,
                        box_radius
                    )
                    sdf.fill(progress_in_bg);
                }
            }
            return sdf.result
        }
    }
}



#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct  DrawGProgress{
    #[deref]
    pub draw_super: DrawQuad,
    #[live(1.0)]
    pub background_visible: f32,
    #[live]
    pub position: f32,
    #[live]
    pub progress_type: GProgressType,
    #[live]
    pub background_color: Vec4, // 盒子的背景色
    #[live]
    pub hover_color: Vec4, // 盒子的hover颜色
    #[live]
    pub focus_color: Vec4, // 盒子的focus颜色
    #[live]
    pub stroke_color: Vec4,
    #[live]
    pub stroke_hover_color: Vec4,
    #[live]
    pub stroke_focus_color: Vec4, 
    #[live]
    pub border_color: Vec4, // 盒子的边框颜色
    #[live(1.0)]
    pub border_width: f32, // 盒子的边框宽度
    #[live(2.0)]
    pub border_radius: f32, // 盒子的圆角半径
    #[live]
    pub hover: f32, // 盒子的hover状态
    #[live]
    pub focus: f32, // 盒子的focus状态
}

#[derive(Live, LiveHook, Clone)]
#[live_ignore]
#[repr(u32)]
pub enum GProgressType {
    #[pick] Horizontal = shader_enum(1),
    Vertical = shader_enum(2),

}

impl DrawGProgress {
    pub fn apply_type(&mut self, progress_type: GProgressType) {
        self.progress_type = progress_type;
    }
}