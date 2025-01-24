use makepad_widgets::Cx;

// pub mod usage;
// pub mod animate;
// pub mod event;
// pub mod virt;

pub fn register(cx: &mut Cx){
    self::live_design(cx);
}

use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;
    
    pub ShaderPage = {{ShaderPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        background_visible: false,
        border_radius: 0.0,
        spacing: 12.0,
        padding: 12.0,
        scroll_bars: <GScrollBars>{},
        clip_x: true,
        clip_y: true,
        <GHLayout>{
            height: Fit,
            align: {x: 0.5},
            <GLabel>{
                font_size: 14.0,
                font_family: (BOLD_FONT),
                text: "Shader",
            }
            <GLabel>{
                width: Fill,
                text: "Shader is a component for drawing custom graphics, you can use self.opened to control the shader(if you need to use self.time).",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                sd = <GShader>{
                    height: 200.0,
                    width: 200.0,
                    animation_key: false,
                    draw_shader:{
                        fn draw(self) -> vec4 {
                                        
                            let uv = self.pos - 0.5;
                            
                            let time = mix(1.0, self.time * 0.5, self.opened);
        
                            let col = vec3(0.0);
                            let noise = fract(sin(dot(uv, vec2(12.9898, 78.233))) * 43758.5453);
                            
                            col += 0.1 * vec3(noise, noise* 0.5, noise * 0.2);
        
                            let r = length(uv);
                            let a = atan(uv.y, uv.x);
                            let f = 0.5 + 0.5 * sin(6.0 * (a + time) + r * 10.0);
                            col += vec3(f* 0.3, f* 0.2, f* 0.5);
                            let i = 0;
                            for _i in 0..10 {
                                // let x = sin(float(i)) * 0.1 + time;
                                // let star_posi = vec2(fract( * 0.1 + time), fract(sin(float(i) * 23421.6313) * 0.1 + time));
                                let star_pos = vec2(fract(sin(float(i) * 43758.5453) * 0.1 + time), fract(sin(float(i) * 23421.6313) * 0.1 + time));
                                star_pos = star_pos * 2.0 - 1.0;
                                star_pos.x *= uv.x / uv.y;
                                let d = uv - star_pos;
                                let star = 1.0 / length(d) * 0.05;
                                col += vec3(star);
                                i = i + 1;
                            }
                            let flicker = fract(sin(dot(uv.xy + time * 20.0, vec2(12.9898,78.233))) * 43758.5453);
                            col += 0.1 * vec3(flicker, flicker * 0.5, flicker * 0.2);
        
                            let plasma = sin(uv.x * 10.0 + time * 2.0) * cos(uv.y * 10.0 + time * 2.0); 
                            col += vec3(0.2, 0.1, 0.3) * plasma;
                            
                            let morph = sin(time + r * 10.0) * 0.5 + 0.5;
                            col *= vec3(morph, morph * 0.8, morph * 1.2);
        
                            return vec4(col, 1.0);
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    spacing: 16.0,
                    open_btn = <GButton>{
                        slot: {
                            text: "Open Shader",
                        }
                    }
                    close_btn = <GButton>{
                        slot: {
                            text: "Close Shader",
                        }
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 300.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                sd = <GShader>{
                    height: 200.0,
                    width: 200.0,
                    animation_key: false,
                    draw_shader:{
                        fn draw(self) -> vec4 {
                                        
                            let uv = self.pos - 0.5;
                            
                            let time = mix(1.0, self.time * 0.5, self.opened);
        
                            let col = vec3(0.0);
                            let noise = fract(sin(dot(uv, vec2(12.9898, 78.233))) * 43758.5453);
                            
                            col += 0.1 * vec3(noise, noise* 0.5, noise * 0.2);
        
                            let r = length(uv);
                            let a = atan(uv.y, uv.x);
                            let f = 0.5 + 0.5 * sin(6.0 * (a + time) + r * 10.0);
                            col += vec3(f* 0.3, f* 0.2, f* 0.5);
                            let i = 0;
                            for _i in 0..10 {
                                // let x = sin(float(i)) * 0.1 + time;
                                // let star_posi = vec2(fract( * 0.1 + time), fract(sin(float(i) * 23421.6313) * 0.1 + time));
                                let star_pos = vec2(fract(sin(float(i) * 43758.5453) * 0.1 + time), fract(sin(float(i) * 23421.6313) * 0.1 + time));
                                star_pos = star_pos * 2.0 - 1.0;
                                star_pos.x *= uv.x / uv.y;
                                let d = uv - star_pos;
                                let star = 1.0 / length(d) * 0.05;
                                col += vec3(star);
                                i = i + 1;
                            }
                            let flicker = fract(sin(dot(uv.xy + time * 20.0, vec2(12.9898,78.233))) * 43758.5453);
                            col += 0.1 * vec3(flicker, flicker * 0.5, flicker * 0.2);
        
                            let plasma = sin(uv.x * 10.0 + time * 2.0) * cos(uv.y * 10.0 + time * 2.0); 
                            col += vec3(0.2, 0.1, 0.3) * plasma;
                            
                            let morph = sin(time + r * 10.0) * 0.5 + 0.5;
                            col *= vec3(morph, morph * 0.8, morph * 1.2);
        
                            return vec4(col, 1.0);
                        }
                    }
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
                    let mut sd = self.gshader(id!(sd));
                    let open_btn = self.gbutton(id!(open_btn));
                    let close_btn = self.gbutton(id!(close_btn));

                    if open_btn.clicked(&actions).is_some(){
                        sd.open(cx);
                    }
                    if close_btn.clicked(&actions).is_some(){
                        sd.close(cx);
                    }
                }
                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct ShaderPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ShaderPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ShaderPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut sd = self.gshader(id!(sd));
        let open_btn = self.gbutton(id!(open_btn));
        let close_btn = self.gbutton(id!(close_btn));

        if open_btn.clicked(&actions).is_some(){
            sd.open(cx);
        }
        if close_btn.clicked(&actions).is_some(){
            sd.close(cx);
        }
    }
}
