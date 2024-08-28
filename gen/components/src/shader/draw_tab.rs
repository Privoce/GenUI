use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawTabBtn = {{DrawTabBtn}}{
        instance inset: vec4(0.0, 0.0, 0.0, 0.0)
        instance hover: 0.0,
        instance pressed: 0.0,
        
        fn get_color(self) -> vec4 {
            return mix(
                mix(
                    self.background_color,
                    self.hover_color,
                    self.hover
                ),
                self.pressed_color,
                self.pressed
            )
        }

        fn get_border_color(self) -> vec4 {
            return self.border_color
        }

        fn get_border_width(self) -> float {
            return self.border_width
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);

            if self.selected == 1.0{
                if self.plain == 1.0{
                    let height = 3.0;
                    sdf.rect(
                        self.inset.x + self.border_width,
                        self.rect_size.y - self.border_width - height,
                        self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                        height
                    );
                    sdf.fill(self.background_color);
                }else{
                    sdf.box(
                        self.inset.x + self.border_width,
                        self.inset.y + self.border_width,
                        self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                        self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                        max(1.0, self.border_radius)
                    );
                   sdf.fill_keep(self.background_color);
                }
            }
            sdf.stroke(self.get_border_color(), self.border_width);
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawTabBtn{
    #[deref] pub draw_super: DrawQuad,
    #[live] pub background_color: Vec4,
    #[live] pub border_color: Vec4,
    #[live(0.0)] pub border_width: f32,
    #[live(4.0)] pub border_radius: f32,
    #[live] pub hover_color: Vec4,
    #[live] pub pressed_color: Vec4,
    #[live(0.0)] pub selected: f32,
    #[live(0.0)] pub plain: f32,
    #[live(1.0)] pub scale: f32,
    #[live(1.0)] pub opacity: f32,
    #[live(0.0)] pub rotation: f32
}