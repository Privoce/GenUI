use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawGView = {{DrawGView}}{              
        varying rect_size2: vec2,
        varying rect_size3: vec2,
        varying rect_pos2: vec2,     
        varying rect_shift: vec2,    
        varying sdf_rect_pos: vec2,
        varying sdf_rect_size: vec2,
                              
        fn get_background_color(self) -> vec4 {
            return mix(
                mix(
                    self.background_color,
                    self.hover_color,
                    self.hover
                ),
                self.focus_color,
                self.focus
            );
        }
                            
        fn vertex(self) -> vec4 {
            let min_offset = min(self.shadow_offset,vec2(0));
            self.rect_size2 = self.rect_size + 2.0*vec2(self.get_spread_radius());
            self.rect_size3 = self.rect_size2 + abs(self.shadow_offset);
            self.rect_pos2 = self.rect_pos - vec2(self.get_spread_radius()) + min_offset;
            self.sdf_rect_size = self.rect_size2 - vec2(self.get_spread_radius() * 2.0 + self.border_width * 2.0)
            self.sdf_rect_pos = -min_offset + vec2(self.border_width + self.get_spread_radius());
            self.rect_shift = -min_offset;
            if self.get_spread_radius() != 0.0{
                return self.clip_and_transform_vertex(self.rect_pos2, self.rect_size3);
            }else{
                return self.clip_and_transform_vertex(self.rect_pos, self.rect_size);
            }
        }
                                        
        fn get_border_color(self) -> vec4 {
            return self.border_color;
        }

        fn get_spread_radius(self) -> float{
            return self.spread_radius;
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
            if self.get_spread_radius() != 0.0 {
                if sdf.shape > -1.0{
                    let m = self.blur_radius;
                    let o = self.shadow_offset + self.rect_shift;
                    if self.border_radius != 0.0 {
                        let v = GaussShadow::rounded_box_shadow(vec2(m) + o, self.rect_size2 + o, self.pos * (self.rect_size3+vec2(m)), self.get_spread_radius() * 0.5, self.border_radius*2.0);
                        let shadow_color = vec4(self.shadow_color.rgb, self.shadow_color.a * v); 
                        sdf.clear(shadow_color);
                    }else{
                        let v = GaussShadow::box_shadow(vec2(m) + o, self.rect_size2 + o, self.pos * (self.rect_size3+vec2(m)), self.get_spread_radius() * 0.5);
                        let shadow_color = vec4(self.shadow_color.rgb, self.shadow_color.a * v); 
                        sdf.clear(shadow_color);
                    }
                }
            }
                                   
            sdf.stroke(self.get_border_color(), self.border_width); 
            
            return sdf.result
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGView{
    #[deref] pub draw_super: DrawQuad,
    #[live] pub background_color: Vec4,
    #[live] pub border_color: Vec4,
    #[live(0.0)] pub border_width: f32,
    #[live(0.0)] pub border_radius: f32,
    #[live] pub hover_color: Vec4,
    #[live] pub focus_color: Vec4,
    #[live] pub shadow_color: Vec4,
    #[live(0.0)] pub spread_radius: f32,
    #[live(4.8)] pub blur_radius: f32,
    #[live] pub shadow_offset: Vec2,
    #[live(0.0)] pub background_visible: f32,
    #[live(1.0)] pub scale: f32,
    #[live(1.0)] pub opacity: f32,
    #[live(0.0)] pub rotation: f32,
    #[live] pub hover: f32,
    #[live] pub focus: f32,
}