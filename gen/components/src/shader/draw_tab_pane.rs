use makepad_widgets::*;

use super::draw_view::DrawGView;

live_design!{
    import makepad_draw::shader::std::*;
    DrawTabPane = {{DrawTabPane}}{
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
            let sdf = Sdf2d::viewport(self.pos * self.rect_size)
            // sdf.rect(
            //     self.inset.x + self.border_width,
            //     0.0,
            //     self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
            //     3.0
            // );
            sdf.fill(self.get_color())
            sdf.box(
                self.inset.x + self.border_width,
                self.inset.y + self.border_width,
                self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                max(1.0, self.border_radius)
            )
            if self.background_visible == 0.0 {
               sdf.fill_keep(self.get_color())
            }
            
            sdf.stroke(self.get_border_color(), self.border_width)
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawTabPane{
    #[deref] pub draw_super: DrawGView,
   
}