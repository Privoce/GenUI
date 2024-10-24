use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawGDivider = {{DrawGDivider}}{
        fn get_stroke_color(self) -> vec4 {
            return mix(
                self.stroke_color,
                self.stroke_hover_color,
                self.hover
            )
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size)

            sdf.box(
                self.pos.x,
                self.pos.y,
                self.rect_size.x,
                self.rect_size.y,
                0.0
            )

            sdf.stroke(#FFFFFFFF, 0.0)

            sdf.move_to(0.0, self.rect_size.y * 0.5);
            sdf.line_to(self.rect_size.x, self.rect_size.y * 0.5);
            sdf.stroke(self.get_stroke_color(), self.stroke_width);

            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGDivider{
    #[deref] pub draw_super: DrawQuad,
    #[live] pub stroke_color: Vec4,
    #[live] pub stroke_hover_color: Vec4,
    #[live] pub hover: f32,
    #[live(0.0)] pub stroke_width: f32,
}