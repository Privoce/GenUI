use makepad_widgets::*;
live_design! {
    DrawGSvg = {{DrawGSvg}}{
        fn get_color(self) -> vec4 {
            return mix(
                mix(
                    self.color,
                    self.stroke_hover_color,
                    self.hover
                ),
                self.stroke_focus_color,
                self.focus
            );
        }
        
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGSvg {
    #[deref]
    pub draw_super: DrawIcon,
    #[live]
    pub stroke_hover_color: Vec4,
    #[live]
    pub stroke_focus_color: Vec4,
    #[live(0.0)]
    pub hover: f32,
    #[live(0.0)]
    pub focus: f32,
}

impl DrawGSvg {
    pub fn set_src(&mut self, src: LiveDependency) -> () {
        self.svg_file = src;
    }
    pub fn area(&self) -> Area{
        self.draw_super.draw_vars.area
    }
}