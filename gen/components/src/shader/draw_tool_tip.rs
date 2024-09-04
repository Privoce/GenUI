use makepad_widgets::*;

live_design!{
    DrawSuperTip = {{DrawSuperTip}}{
        fn pixel(self) ->vec4{
            return #FF0000;
        }
    }
    DrawToolTip = {{DrawToolTip}}{
        fn pixel(self) ->vec4{
            let stroke_width = self.stroke_width;
            return vec4(1.0);
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
pub struct DrawSuperTip{
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub stroke_width: f32,
}

#[derive(Live, LiveHook, LiveRegister)]
pub struct DrawToolTip{
    #[deref]
    pub draw_super: DrawSuperTip,
    #[live]
    pub background_color: Vec4,
}