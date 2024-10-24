use makepad_widgets::*;

use super::draw_view::DrawGView;

live_design! {
    import makepad_draw::shader::std::*;
    DrawGLink = {{DrawGLink}}{
        fn get_underline_color(self) -> vec4 {
            return mix(
                mix(
                    self.underline_color,
                    self.underline_hover_color,
                    self.hover
                ),
                self.underline_focus_color,
                self.focus
            )
        }
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size3);
            sdf.box(
                self.sdf_rect_pos.x,
                self.sdf_rect_pos.y,
                self.sdf_rect_size.x,
                self.sdf_rect_size.y,
                max(1.0, self.border_radius)
            );

            if self.background_visible != 0.0 {
                sdf.fill_keep(self.get_background_color());
            }
            if self.spread_radius != 0.0 {
                if sdf.shape > -1.0{
                    let m = self.blur_radius;
                    let o = self.shadow_offset + self.rect_shift;
                    if self.border_radius != 0.0 {
                        let v = GaussShadow::rounded_box_shadow(vec2(m) + o, self.rect_size2 + o, self.pos * (self.rect_size3+vec2(m)), self.spread_radius * 0.5, self.border_radius*2.0);
                        let shadow_color = vec4(self.shadow_color.rgb, self.shadow_color.a * v);
                        sdf.clear(shadow_color);
                    }else{
                        let v = GaussShadow::box_shadow(vec2(m) + o, self.rect_size2 + o, self.pos * (self.rect_size3+vec2(m)), self.spread_radius * 0.5);
                        let shadow_color = vec4(self.shadow_color.rgb, self.shadow_color.a * v);
                        sdf.clear(shadow_color);
                    }
                }
            }

            sdf.stroke(self.get_border_color(), self.border_width);
            let offset = self.underline_width;
            sdf.move_to(0., self.rect_size.y - offset);
            sdf.line_to(self.rect_size.x, self.rect_size.y - offset);
            // sdf.stroke(self.get_border_color(), self.border_width)
            if self.underline_visible == 1.0 {
                sdf.stroke(self.get_underline_color(), self.underline_width);
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGLink {
    #[deref]
    pub draw_super: DrawGView,
    #[live(1.0)]
    pub underline_visible: f32,
    #[live]
    pub underline_color: Vec4,
    #[live]
    pub underline_hover_color: Vec4,
    #[live]
    pub underline_focus_color: Vec4,
    #[live(1.0)]
    pub underline_width: f32,
    // #[live(1.0)]
    // pub underline_offset: f32,
}
