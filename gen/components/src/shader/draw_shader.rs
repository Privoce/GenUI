use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawGShader = {{DrawGShader}}{
        fn draw(self) -> vec4{
            return vec4(1.0);
        }

        fn pixel(self) -> vec4{
            return mix(
                vec4(1.0),
                self.draw(),
                self.opened
            );
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGShader{
    #[deref] pub draw_super: DrawQuad,
    #[live] pub opened: f32,
}