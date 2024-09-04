use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawToolTip = {{DrawToolTip}}{
        fn pixel(self) ->vec4{
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let spacing = 1.0;
            let counter = 0.0;
            for i in 0..500{
                counter += 1.0;
                sdf.rect(spacing * counter, 0.0, spacing, self.rect_size.y);
                sdf.rect(0.0, spacing * counter, self.rect_size.y, spacing);
            }
            
            sdf.stroke(vec4(0.0, 0.0, 0.0, 1.0), 1.0);
            return sdf.result;
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
pub struct DrawToolTip{
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub background_color: Vec4,
}
