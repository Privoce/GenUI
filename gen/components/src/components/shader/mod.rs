pub mod event;
mod register;

use event::*;
use makepad_widgets::*;
pub use register::register;

use crate::{event_bool, ref_event_bool, set_event_bool, widget_area};

live_design! {
    GShaderBase = {{GShader}} {}
}

#[derive(Live, Widget)]
pub struct GShader {
    #[redraw]
    #[live]
    pub draw_shader: DrawQuad,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live]
    pub time: f32,
    #[rust]
    next_frame: NextFrame,
    // store previous state(animation_open)
    #[rust]
    pub pre_state: bool,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub animation_open: bool,
}

impl LiveHook for GShader {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // starts the animation cycle on startup
        if self.animation_open {
            self.next_frame = cx.new_next_frame();
        }
    }
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.pre_state = self.animation_open;
        if !self.visible {
            return;
        }
    }
    fn after_update_from_doc(&mut self, cx: &mut Cx) {
        if self.pre_state != self.animation_open {
            let uid = self.widget_uid();
            if self.pre_state {
                cx.widget_action(uid, &Scope::empty().path, GShaderEvent::Close);
            } else {
                cx.widget_action(uid, &Scope::empty().path, GShaderEvent::Open);
            }
        }
    }
}

impl Widget for GShader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.animation_open || !self.visible {
            return;
        }
        if let Some(ne) = self.next_frame.is_event(event) {
            // update time to use for animation
            self.time = (ne.time * 0.001).fract() as f32;

            // force updates, so that we can animate in the absence of user-generated events
            self.redraw(cx);
            self.next_frame = cx.new_next_frame();
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.draw_shader.begin(cx, walk, self.layout);
        self.draw_shader.end(cx);
        DrawStep::done()
    }
}

impl GShader {
    widget_area! {
        area, draw_shader
    }
    event_bool!{
        open: GShaderEvent::Open,
        close: GShaderEvent::Close
    }
}

impl GShaderRef{
    ref_event_bool! {
        open,
        close
    }
}

impl GShaderSet {
    set_event_bool!{
        open, 
        close
    }
}