use makepad_widgets::*;
live_design! {
    import makepad_draw::shader::std::*;

    DrawGIconPixel = {{DrawGIconPixel}}{
        
        fn pixel(self) -> vec4{
            return vec4(1.0);
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGIconPixel {
    #[deref]
    pub draw_super: DrawQuad,
    // #[live]
    // pub hover_color: Vec4,
    // #[live(0.0)]
    // pub hover: f32,
    #[live(1.0)] pub brightness: f32,
    #[live(0.6)] pub curve: f32,
    #[live(0.5)] pub linearize: f32,
    #[live] pub color: Vec4,
    // #[live(1.0)] pub scale: f64,
}