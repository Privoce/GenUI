use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawGDivider = {{DrawGDivider}}{
        instance inset: vec4(0.0, 0.0, 0.0, 0.0)
        instance hover: 0.0,
        
        fn get_color(self) -> vec4 {
            return mix(
                self.color,
                self.hover_color,
                self.hover
            )
        }

      
       
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size)

            sdf.box(
                self.inset.x,
                self.inset.y,
                self.rect_size.x - (self.inset.x + self.inset.z),
                self.rect_size.y - (self.inset.y + self.inset.w),
                0.0
            )
            // if self.background_visible == 0.0 {
            //    sdf.fill_keep(self.get_color())
            // }
            sdf.stroke(#FFFFFFFF, 0.0)

            sdf.move_to(0.0, self.rect_size.y * 0.5);
            sdf.line_to(self.rect_size.x, self.rect_size.y * 0.5);
            sdf.stroke(self.get_color(), self.stroke_width);

            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGDivider{
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub hover_color: Vec4,
    #[live(0.0)] pub stroke_width: f32,
}