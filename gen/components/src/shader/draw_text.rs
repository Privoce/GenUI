use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;

    DrawGText = {{DrawGText}} {
        
        fn get_color(self) -> vec4 {
            // return mix(
            //     self.color,
            //     mix(self.hover_color, self.pressed_color, self.pressed),
            //     self.hover
            // )
            return mix(
                mix(
                    self.color,
                    self.hover_color,
                    self.hover
                ),
                self.pressed_color,
                self.pressed
            )
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGText {
    #[deref]
    pub draw_super: DrawText,
    #[live]
    pub hover_color: Vec4,
    #[live]
    pub pressed_color: Vec4,
    // text is empty or not
    #[live]
    pub empty: f32,
    #[live]
    pub hover: f32,
    #[live]
    pub pressed: f32
}

impl DrawGText {
    pub fn is_empty(&self) -> bool {
        self.empty == 1.0
    }
    pub fn set_empty(&mut self, empty: bool) {
        self.empty = if empty { 1.0 } else { 0.0 }
    }
}
