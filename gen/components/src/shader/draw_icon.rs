use makepad_widgets::*;
live_design! {
    DrawGSvg = {{DrawGSvg}}{
        fn get_color(self) -> vec4 {
            return mix(
                self.color,
                self.hover_color,
                self.hover
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
    pub hover_color: Vec4,
    #[live(0.0)]
    pub hover: f32,
}

impl DrawGSvg {
    pub fn set_src(&mut self, src: LiveDependency) -> () {
        self.svg_file = src;
    }
    pub fn area(&self) -> Area{
        self.draw_super.draw_vars.area
    }
}